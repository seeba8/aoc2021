
pub fn solve(mut input: impl Iterator<Item = i64>) -> Option<(i64, i64, i64, i64)> {
    let mut w: i64;
    let mut z: i64;

    // 9
    // 3
    z = input.next()? + 10;

    // x8
    // 31
    w = input.next()?;
    //if (z % 26) + 11 != w {  // aways true
    z *= 26;
    z += w + 16;
    //}
    
    // xx9
    // 315
    w = input.next()?;
    //if (z % 26) + 11 != w { // always true
    z *= 26;
    z += w;
    //}

    // xx99
    // 3152
    w = input.next()?;
    //if (z % 26) + 10 != w { // always true
    z *= 26;
    z += w + 13;
    //}

    // xx998
    // 31521
    w = input.next()?;
    if (z % 26) - 14 != w {
        z = (z / 26) * 26;
        z += w + 7;
    } else {
        z /= 26;
    }
    // xx9985
    // 315211
    w = input.next()?;
    if (z % 26) - 4 != w {
        z = (z / 26) * 26;
        z += w + 11;
    } else {
        z /= 26;
    }

    // xx99851
    // 3152111
    w = input.next()?;
    //if (z % 26) + 11 != w { // always true
    z *= 26;
    z += w + 11;
    //}

    // xx998519
    // 31521119
    w = input.next()?;
    if (z % 26) - 3 != w {
        z = (z / 26) * 26;
        z += w + 10;
    } else {
        z /= 26;
    }

    // xx9985195
    // 315211191
    w = input.next()?;
    //if (z % 26) + 12 != w { // always true
    z *= 26;
    z += w + 16;
    //}
    
    // xx99851959
    // 3152111915
    w = input.next()?;
    if (z % 26) - 12 != w {
        z = (z / 26) * 26;
        z += w + 8;
    } else {
        z /= 26;
    }

    // xx998519596
    // 31521119151
    w = input.next()?;
    //if (z % 26) + 13 != w { // always true
    z *= 26;
    z += w + 15;
    //}

    // xx9985195969
    // 315211191514
    w = input.next()?;
    if (z % 26) - 12 != w {
        z = (z / 26) * 26;
        z += w + 2;
    } else {
        z /= 26;
    }

    // x899851959699
    // 3152111915142
    w = input.next()?;
    if (z % 26) - 15 != w {
        z = (z / 26) * 26;
        z += w + 5;
    } else {
        z /= 26;
    }

    // 98998519596997
    // 31521119151421
    // last digit + 12 = z
    w = input.next()?;
    if (z % 26) - 12 != w {
        z = (z / 26) * 26;
        z += w + 10;
    } else {
        z /= 26;
    }
    Some((w, 0, 0, z))
}
