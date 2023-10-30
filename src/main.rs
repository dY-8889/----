type Formula = Vec<Vec<String>>;
type Fraction = Vec<String>;
type Answer = Vec<i128>;

// Vec<String>をAnswerに変換
fn as_num(value: String) -> i128 {
    value.trim().parse().unwrap()
}

// AnswerをVec<String>に変換
fn as_str(value: Answer) -> Vec<String> {
    value.iter().map(|x| x.to_string()).collect()
}

// 約分
fn common_divisor(frac: Vec<String>) -> Answer {
    let mut frac = vec![as_num(frac[0].clone()), as_num(frac[1].clone())];
    let mut answer: Answer = vec![];
    let mut buf: i128 = 0;

    frac.sort();
    println!("{:?}", frac);
    for i in 1..frac[1] - 1 {
        if frac[0] % i == 0 && frac[1] % i == 0 {
            buf = i;
        }
    }

    answer.push(frac[0] / buf);
    answer.push(frac[1] / buf);

    return answer;
}

// 最小公倍数
fn common_multiple(parameter1: i128, parameter2: i128) -> i128 {
    let mut buf1 = vec![parameter1];
    let mut buf2 = vec![parameter2];

    let mut i = 2;
    loop {
        buf1.push(buf1[0] * i);
        buf2.push(buf2[0] * i);

        for iter in buf1.iter() {
            if let Some(_) = buf2.iter().position(|&item| item == *iter) {
                return *iter;
            }
        }
        i += 1;
    }
}
// 計算
fn cal(num1: Fraction, num2: Fraction, symbol: String) -> Answer {
    let mut num1: Vec<String> = num1.clone();
    let mut num2: Vec<String> = num2.clone();

    let mut answer: Answer = Vec::new();

    if num1.len() == 1 {
        num1.insert(0, "1".to_string())
    }
    if num2.len() == 1 {
        num2.insert(0, "1".to_string())
    }

    println!("{:?}", num1);
    println!("{:?}", num2);

    match symbol.as_str() {
        "*" => {
            answer.push(as_num(num1[0].clone()) * as_num(num2[0].clone()));
            answer.push(as_num(num1[1].clone()) * as_num(num2[1].clone()));
        }
        "/" => {
            answer.push(as_num(num1[0].clone()) * as_num(num2[1].clone()));
            answer.push(as_num(num1[1].clone()) * as_num(num2[0].clone()));
        }
        "+" => {
            let com_mul = common_multiple(as_num(num1[0].clone()), as_num(num2[0].clone()));
            println!("{:?}", com_mul);
            let num1_com_mul = com_mul / as_num(num1[0].clone());
            let num2_com_mul = com_mul / as_num(num2[0].clone());
            println!("{:?}", num1_com_mul);
            println!("{:?}", num2_com_mul);
            answer.push(com_mul);
            answer.push(
                (num1_com_mul * as_num(num1[1].clone())) + (num2_com_mul * as_num(num2[1].clone())),
            )
        }
        "-" => {
            let com_mul = common_multiple(as_num(num1[0].clone()), as_num(num2[0].clone()));
            println!("{:?}", com_mul);
            let num1_com_mul = com_mul / as_num(num1[0].clone());
            let num2_com_mul = com_mul / as_num(num2[0].clone());
            println!("{:?}", num1_com_mul);
            println!("{:?}", num2_com_mul);
            answer.push(com_mul);
            answer.push(
                (num1_com_mul * as_num(num1[1].clone())) - (num2_com_mul * as_num(num2[1].clone())),
            )
        }
        _ => (),
    }

    answer
}

fn main() {
    // 式
    let mut list: Formula = vec![
        vec!["3".to_string(), "2".to_string()],
        vec!["+".to_string()],
        vec!["5".to_string()],
        vec!["*".to_string()],
        vec!["6".to_string(), "6".to_string()],
    ];

    // 先に掛け算割り算
    'main: loop {
        for i in (1..list.len()).step_by(2) {
            if list[i][0] == "*" || list[i][0] == "/" {
                // 記号の前後の数値を取得
                let num1: Fraction = list[i - 1].clone();
                let num2: Fraction = list[i + 1].clone();

                let answer: Answer = cal(num1, num2, list[i][0].clone());

                println!("{:?}", answer);
                // 計算した式を削除
                list.drain(i - 1..=i + 1);
                // 結果を追加
                list.insert(i - 1, as_str(answer));
                continue 'main;
            }
        }

        println!("{:?}", list);
        break;
    }
    'main: loop {
        for i in (1..list.len()).step_by(2) {
            // 記号の前後の数値を取得
            let num1: Fraction = list[i - 1].clone();
            let num2: Fraction = list[i + 1].clone();

            let answer: Answer = cal(num1, num2, list[i][0].clone());

            println!("{:?}", answer);
            // 計算した式を削除
            list.drain(i - 1..=i + 1);
            // 結果を追加
            list.insert(i - 1, as_str(answer));

            continue 'main;
        }

        println!("{:?}", list);
        break;
    }
    println!("{:?}", common_divisor(list[0].clone()));
}
