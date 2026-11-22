package com.example;

import java.util.List;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.server.ResponseStatusException;

@RestController
@RequestMapping("/api/users")
public class UserController {
    private final UserService service;

    public UserController(UserService service) {
        this.service = service;
    }

    @GetMapping
    public List<User> list(@RequestParam(required = false) String name) {
        if (name != null) {
            return service.byName(name);
        }
        return service.all();
    }

    @GetMapping("/{id}")
    public ResponseEntity<User> get(@PathVariable("id") Long id) {
        User u = service.find(id).orElse(null);
        if (u == null) {
            return ResponseEntity.notFound().build();
        }
        return ResponseEntity.ok(u);
    }

    @PostMapping
    @ResponseStatus(HttpStatus.CREATED)
    public User create(@RequestBody User user) {
        try {
            return service.create(user);
        } catch (IllegalArgumentException e) {
            throw new ResponseStatusException(HttpStatus.BAD_REQUEST, e.getMessage());
        }
    }

    @GetMapping("/grade")
    public String grade(@RequestParam(defaultValue = "0") int score) {
        return service.grade(score);
    }

    @PostMapping("/sum")
    public int sum(@RequestBody List<Integer> xs) {
        return service.sum(xs);
    }

    @GetMapping("/initials")
    public String initials(@RequestParam String name) {
        return service.initials(name);
    }

    @DeleteMapping("/{id}")
    public ResponseEntity<Void> delete(@PathVariable Long id) {
        if (service.delete(id)) {
            return ResponseEntity.noContent().build();
        }
        return ResponseEntity.notFound().build();
    }

    @GetMapping("/total/{n}")
    public int total(@PathVariable int n) {
        int s = 0;
        for (int i = 0; i < n; i++) {
            if (i % 2 == 0) {
                s += i;
            } else {
                s -= 1;
            }
        }
        return s;
    }
}
