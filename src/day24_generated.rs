
#[allow(unused)]
pub fn solve(mut input: impl Iterator<Item = i64>) -> Option<(i64, i64, i64, i64)> {
    let mut w: i64 = 0;
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut z: i64 = 0;
    w = input.next()?;
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 13;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 10;
    y *= x;
    z += y;
    w = input.next()?;
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 11;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 16;
    y *= x;
    z += y;
    w = input.next()?;
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 11;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 0;
    y *= x;
    z += y;
    w = input.next()?;
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 10;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 13;
    y *= x;
    z += y;
    w = input.next()?;
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -14;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 7;
    y *= x;
    z += y;
    w = input.next()?;
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -4;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 11;
    y *= x;
    z += y;
    w = input.next()?;
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 11;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 11;
    y *= x;
    z += y;
    w = input.next()?;
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -3;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 10;
    y *= x;
    z += y;
    w = input.next()?;
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 12;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 16;
    y *= x;
    z += y;
    w = input.next()?;
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -12;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 8;
    y *= x;
    z += y;
    w = input.next()?;
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 13;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 15;
    y *= x;
    z += y;
    w = input.next()?;
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -12;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 2;
    y *= x;
    z += y;
    w = input.next()?;
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -15;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 5;
    y *= x;
    z += y;
    w = input.next()?;
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -12;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 10;
    y *= x;
    z += y;
    Some((w,x,y,z))
}
