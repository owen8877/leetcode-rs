pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if buildings.len() == 0 {
        return vec![]
    }
    let mut skylines = vec![vec![buildings[0][0], buildings[0][2]], vec![buildings[0][1], 0]];
    for building in buildings {
        let L = building[0];
        let R = building[1];
        let H = building[2];

        let mut j = 0;
        let n = skylines.len();
        if R < skylines[0][0] {
            skylines[0][0] = R;
            skylines.insert(0, vec![L, H]);
        } else if L > skylines[n-1][0] {
            skylines.push(vec![L, H]);
            skylines.push(vec![R, 0]);
        } else {
            while j < skylines.len() - 1 {
                let Lj = skylines[j][0];
                let Rj = skylines[j+1][0];
                let Hj = skylines[j][1];

                if Hj >= H || L >= Rj || R <= Lj {
                    j += 1;
                    continue
                }
                if Lj < L {
                    if Rj <= R {
                        skylines[j] = vec![Lj, Hj];
                        skylines.insert(j+1, vec![L, H]);
                        j += 2;
                    } else {
                        skylines[j] = vec![Lj, Hj];
                        skylines.insert(j+1, vec![L, H]);
                        skylines.insert(j+2, vec![R, Hj]);
                        j += 3;
                    }
                } else {
                    if Rj <= R {
                        skylines[j] = vec![Lj, H];
                        j += 1;
                    } else {
                        skylines[j] = vec![Lj, H];
                        skylines.insert(j+1, vec![R, Hj]);
                        j += 2;
                    }
                }
            }
            if skylines[j][0] < R {
                skylines[j] = vec![skylines[j][0], H];
                skylines.push(vec![R, 0]);
            }
            for j in (1..skylines.len()).rev() {
                if skylines[j][1] == skylines[j-1][1] {
                    skylines.remove(j);
                }
            }
        }
    }
    skylines
}