"""Java_to が生成したコードが使う小さな実行時部品(標準ライブラリだけで動く。fastapi は必要な関数の中でだけ読み込む)。"""
from __future__ import annotations

import re
from typing import Any

__all__ = [
    "LoggerFactory",
    "StringBuilder",
    "AtomicNumber",
    "jformat",
    "or_else",
    "or_else_get",
    "or_else_throw",
    "ResponseBuilder",
    "HttpStatus",
    "Status",
    "to_response",
    "InMemoryRepository",
]


class StringBuilder:
    def __init__(self, init: str = "") -> None:
        self._parts: list[str] = [str(init)] if init else []

    def append(self, x: Any) -> "StringBuilder":
        self._parts.append(str(x))
        return self

    def insert(self, i: int, x: Any) -> "StringBuilder":
        s = self.to_string()
        self._parts = [s[:i] + str(x) + s[i:]]
        return self

    def reverse(self) -> "StringBuilder":
        self._parts = [self.to_string()[::-1]]
        return self

    def length(self) -> int:
        return len(self.to_string())

    def to_string(self) -> str:
        return "".join(self._parts)

    def __str__(self) -> str:
        return self.to_string()

    def __len__(self) -> int:
        return self.length()

    def __add__(self, other: Any) -> str:
        return self.to_string() + str(other)

    def __radd__(self, other: Any) -> str:
        return str(other) + self.to_string()


class AtomicNumber:
    def __init__(self, v: int = 0) -> None:
        self.v = v

    def increment_and_get(self) -> int:
        self.v += 1
        return self.v

    def get(self) -> int:
        return self.v

    def set(self, v: int) -> None:
        self.v = v


def jformat(fmt: str, *args: Any) -> str:
    """Java の String.format(%s %d %.2f %n %b)を Python の書式へ。"""
    fmt = fmt.replace("%n", "\n").replace("%b", "%s")
    fmt = re.sub(r"%(\d+)\$", "%", fmt)
    try:
        return fmt % args
    except (TypeError, ValueError):
        return fmt + " " + " ".join(str(a) for a in args)


def or_else(v: Any, d: Any) -> Any:
    return v if v is not None else d


def or_else_get(v: Any, supplier: Any) -> Any:
    return v if v is not None else supplier()


def or_else_throw(v: Any, exc: Any = None) -> Any:
    if v is not None:
        return v
    if exc is None:
        raise LookupError("No value present")
    if callable(exc) and not isinstance(exc, BaseException):
        exc = exc()
    raise exc


class _JLogger:
    """SLF4J の Logger 相当(info/warn/error/debug/trace、`{}` の置換つき)。"""

    def __init__(self, name: str) -> None:
        import logging

        self._l = logging.getLogger(name)

    @staticmethod
    def _fmt(msg: Any, args: tuple) -> str:
        m = str(msg)
        for a in args:
            m = m.replace("{}", str(a), 1)
        return m

    def info(self, msg: Any, *a: Any) -> None:
        self._l.info(self._fmt(msg, a))

    def warn(self, msg: Any, *a: Any) -> None:
        self._l.warning(self._fmt(msg, a))

    def error(self, msg: Any, *a: Any) -> None:
        self._l.error(self._fmt(msg, a))

    def debug(self, msg: Any, *a: Any) -> None:
        self._l.debug(self._fmt(msg, a))

    trace = debug


class LoggerFactory:
    @staticmethod
    def get_logger(c: Any = "app") -> "_JLogger":
        return _JLogger(getattr(c, "__name__", str(c)))

    getLogger = get_logger


class HttpStatus:
    OK = 200
    CREATED = 201
    ACCEPTED = 202
    NO_CONTENT = 204
    BAD_REQUEST = 400
    UNAUTHORIZED = 401
    FORBIDDEN = 403
    NOT_FOUND = 404
    CONFLICT = 409
    UNPROCESSABLE_ENTITY = 422
    INTERNAL_SERVER_ERROR = 500
    NOT_IMPLEMENTED = 501


Status = HttpStatus  # JAX-RS の Response.Status


_KIND = {"ok": 200, "created": 201, "accepted": 202, "noContent": 204, "badRequest": 400, "notFound": 404}


class ResponseBuilder:
    """ResponseEntity / Response のビルダー。`build()` `body()` `entity()` はそのまま続けて書ける。"""

    def __init__(self, kind: str, arg: Any = None) -> None:
        self.status = arg if kind == "status" and isinstance(arg, int) else _KIND.get(kind, 200)
        self.payload: Any = None
        if kind == "badRequest" and arg is not None:
            self.payload = arg
        self.headers: dict[str, str] = {}

    def body(self, x: Any) -> "ResponseBuilder":
        self.payload = x
        return self

    entity = body

    def header(self, k: str, v: Any) -> "ResponseBuilder":
        self.headers[str(k)] = str(v)
        return self

    def location(self, uri: Any) -> "ResponseBuilder":
        self.headers["location"] = str(uri)
        return self

    def build(self) -> "ResponseBuilder":
        return self


def to_response(x: Any) -> Any:
    """ResponseBuilder を実際の応答へ。それ以外はそのまま FastAPI に返させる。"""
    if isinstance(x, ResponseBuilder):
        from fastapi.encoders import jsonable_encoder
        from fastapi.responses import JSONResponse, Response

        if x.status == 204 or x.payload is None:
            return Response(status_code=x.status, headers=x.headers)
        return JSONResponse(status_code=x.status, content=jsonable_encoder(x.payload), headers=x.headers)
    return x


class InMemoryRepository:
    """Spring Data の JpaRepository 相当の最小の置き換え(メモリ上)。`findByXxx` などの導出クエリを名前から解釈する。
    実際の DB に置き換えるまでの足場。永続化はされない。"""

    def __init__(self) -> None:
        self._items: dict[Any, Any] = {}
        self._next = 1

    # --- CrudRepository ---
    def save(self, e: Any) -> Any:
        i = getattr(e, "id", None)
        if i is None:
            i = self._next
            self._next += 1
            try:
                setattr(e, "id", i)
            except Exception:
                pass
        elif isinstance(i, int) and i >= self._next:
            self._next = i + 1
        self._items[i] = e
        return e

    def save_all(self, es: Any) -> list[Any]:
        return [self.save(e) for e in es]

    def find_by_id(self, i: Any) -> Any:
        return self._items.get(i)

    def find_all(self, *_: Any) -> list[Any]:
        return list(self._items.values())

    def exists_by_id(self, i: Any) -> bool:
        return i in self._items

    def count(self) -> int:
        return len(self._items)

    def delete_by_id(self, i: Any) -> None:
        self._items.pop(i, None)

    def delete(self, e: Any) -> None:
        self._items.pop(getattr(e, "id", None), None)

    def delete_all(self, *_: Any) -> None:
        self._items.clear()

    def flush(self) -> None:
        return None

    # --- 導出クエリ: find_by_user_name / exists_by_email / count_by_status / delete_by_x / find_all_by_x ---
    def _derived(self, name: str, args: list[Any], many: bool = False) -> Any:
        m = re.match(r"^(find_all_by|find_by|get_by|read_by|query_by|exists_by|count_by|delete_by|remove_by)_(.+)$", name)
        if not m:
            raise NotImplementedError(f"TODO(Java_to): 導出クエリ {name} を実装する")
        kind, rest = m.group(1), m.group(2)
        # OrderBy を先に切り離す(`..._order_by_name_desc`)
        order: list[tuple[str, bool]] = []
        om = re.search(r"_order_by_(.+)$", rest)
        if om:
            rest = rest[: om.start()]
            toks = om.group(1).split("_")
            cur: list[str] = []
            for t in toks:
                if t in ("asc", "desc"):
                    order.append(("".join(cur), t == "desc"))
                    cur = []
                else:
                    cur.append(t)
            if cur:
                order.append(("".join(cur), False))
        # 条件は `_or_` で分け、各項を `_and_` で分ける。条件ごとに演算子の接尾辞を見る
        ops = [
            ("_greater_than_equal", lambda x, y: x >= y, 1), ("_less_than_equal", lambda x, y: x <= y, 1),
            ("_greater_than", lambda x, y: x > y, 1), ("_less_than", lambda x, y: x < y, 1),
            ("_not_containing", lambda x, y: str(y) not in str(x), 1), ("_containing", lambda x, y: str(y) in str(x), 1),
            ("_starting_with", lambda x, y: str(x).startswith(str(y)), 1), ("_ending_with", lambda x, y: str(x).endswith(str(y)), 1),
            ("_ignore_case", lambda x, y: str(x).lower() == str(y).lower(), 1),
            ("_not_in", lambda x, y: x not in y, 1), ("_in", lambda x, y: x in y, 1),
            ("_between", lambda x, y: y[0] <= x <= y[1], 2), ("_is_null", lambda x, y: x is None, 0), ("_is_not_null", lambda x, y: x is not None, 0),
            ("_not_like", lambda x, y: not _like(x, y), 1), ("_like", lambda x, y: _like(x, y), 1), ("_not", lambda x, y: x != y, 1),
            ("_true", lambda x, y: x is True, 0), ("_false", lambda x, y: x is False, 0),
        ]

        def _like(x: Any, pat: Any) -> bool:
            rx = "^" + re.escape(str(pat)).replace("%", ".*").replace("_", ".") + "$"
            return re.match(rx, str(x)) is not None

        conds: list[list[tuple[str, Any, int]]] = []
        for grp in rest.split("_or_"):
            cs = []
            for p in grp.split("_and_"):
                for suf, fn, n in ops:
                    if p.endswith(suf):
                        cs.append((p[: -len(suf)], fn, n))
                        break
                else:
                    cs.append((p, (lambda x, y: x == y), 1))
            conds.append(cs)

        def key(p: str) -> str:
            return p.replace("_", "").lower()

        def attr_of(e: Any, p: str) -> Any:
            for attr in dir(e):
                if not attr.startswith("_") and key(attr) == key(p):
                    return getattr(e, attr)
            return None

        def match(e: Any) -> bool:
            i = 0
            for cs in conds:
                ok = True
                for p, fn, n in cs:
                    a = args[i : i + n]
                    i += n
                    if ok:
                        ok = fn(attr_of(e, p), a[0] if n == 1 else (a if n == 2 else None))
                if ok:
                    return True
            return False

        hits = [e for e in self._items.values() if match(e)]
        for name_, desc in reversed(order):
            hits.sort(key=lambda e, n=name_: (attr_of(e, n) is None, attr_of(e, n)), reverse=desc)
        if kind.startswith("exists"):
            return bool(hits)
        if kind.startswith("count"):
            return len(hits)
        if kind.startswith(("delete", "remove")):
            for e in hits:
                self.delete(e)
            return len(hits)
        if kind == "find_all_by" or many:
            return hits
        return hits[0] if hits else None
