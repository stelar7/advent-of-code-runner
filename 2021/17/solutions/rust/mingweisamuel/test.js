(() => {
    let xb = [217, 240];
    let yb = [-126, -69];

    let vx = 0;
    for (; ; vx++) {
        if (xb[0] < (vx * vx + vx) / 2) {
            break;
        }
    }

    let vy = undefined;
    let y_max = Number.NEGATIVE_INFINITY;
    for (let test_vy = 100; test_vy < 200; test_vy++) {
        let y_max_attempt = ((vx, vy) => {
            console.log('vy=', vy);

            let x = 0;
            let y = 0;
            let y_max = 0;
            while (yb[0] < y) {
                x += vx;
                y += vy;
                vx -= Math.sign(vx);
                vy -= 1;
                y_max = Math.max(y_max, y);

                // console.log(x, y);

                if (xb[0] <= x && x <= xb[1] && yb[0] <= y && y <= yb[1]) {
                    return y_max;
                }
            }
            return Number.NEGATIVE_INFINITY;
        })(vx, test_vy);

        if (y_max < y_max_attempt) {
            vy = test_vy;
            y_max = y_max_attempt;
        }
        // 		else if (false === y_max_attempt && false !== y_max) {
        // 			vy--;
        // 			break;
        // 		}
    }
    console.log(vx, vy, y_max);
})();



(() => {
    //   let xb = [20,30];
    //   let yb = [-10,-5];
    let xb = [217, 240];
    let yb = [-126, -69];

    // 	let vx = 0;
    // 	for (;; vx++) {
    // 		if (xb[0] < (vx * vx + vx) / 2) {
    // 			break;
    // 		}
    //   }

    let count = 0;
    for (let vx = 0; vx < 250; vx++) {
        console.log('vx=', vx);
        for (let vy = -200; vy < 200; vy++) {
            let log = false;
            if (vx == 25 && vy == -7) {
                log = true;
            }
            let y_max_attempt = ((vx, vy) => {
                let x = 0;
                let y = 0;
                let y_max = 0;
                while (yb[0] <= y) {
                    x += vx;
                    y += vy;
                    vx -= Math.sign(vx);
                    vy -= 1;
                    y_max = Math.max(y_max, y);

                    if (log) console.log(x, y);

                    if (xb[0] <= x && x <= xb[1] && yb[0] <= y && y <= yb[1]) {
                        return y_max;
                    }
                }
                return Number.NEGATIVE_INFINITY;
            })(vx, vy);
            if (0 <= y_max_attempt) {
                console.log(vx, vy);
                count++;
            }
        }
        console.log(count);
    }
    console.log(count);
})();