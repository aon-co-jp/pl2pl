program f1
  implicit none
  integer :: i, s = 0
  do i = 10, 1, -3
    s = s + i
  end do
  if (s .gt. 5) then
    print *, 'sum', s
  else
    print *, 'small'
  end if
  print *, s, i
end program f1
