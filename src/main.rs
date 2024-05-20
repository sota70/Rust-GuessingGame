use io::stdout;
use rand::distributions::uniform::SampleRange;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

/*
ゲームの結果を表す列挙
*/
enum GameResult {
    QUIT,
    WIN,
    LOSE,
}

/*
与えられた範囲の乱数を生成し、それを返す関数


param range: 乱数の値の範囲。整数値を想定している。

return: 整数の乱数を返す
 */
fn gen_random_num<R>(range: R) -> i32
where
    R: SampleRange<i32>,
{
    return rand::thread_rng().gen_range(range);
}

/*
ユーザの入力(user_input)と答え(secret_number)が同値かどうか判定する関数


param    user_input: ユーザの予想した値(guessとも言う)
      secret_number: 答えの値

return: guessと答えが同値であればGameResultのWINを、異なる値の場合はGameResultのLOSEを返す
*/
fn guess(user_input: i32, secret_number: i32) -> GameResult {
    match user_input.cmp(&secret_number) {
        Ordering::Equal => {
            println!("You guessed it!");
            return GameResult::WIN;
        }
        Ordering::Greater => {
            println!("Your guess is greater than secret number");
            return GameResult::LOSE;
        }
        Ordering::Less => {
            println!("Your guess is less than secret number");
            return GameResult::LOSE;
        }
    };
}

/*
ユーザが入力した文字列を処理する関数
ユーザからはguessまたはゲーム退出コマンド(q, quit)を期待する
ユーザからguessが得られた場合には、それが答えの値と同値かを確認する関数へ渡す


param    user_input: ユーザの入力した文字列
      secret_number: ゲームの答え

return: ユーザの入力に対する判定を返す。qまたはquitと入力した場合にはGameResult::QUITとなる。
        guessの場合はWINかLOSEを返す。
*/
fn handle_input(user_input: String, secret_number: i32) -> GameResult {
    if user_input == "q\n" || user_input == "quit\n" {
        return GameResult::QUIT;
    }
    // trimを行うのはparse関数で数値に変換する前に、new-lineや空白が入らないようにするため
    let user_guess: i32 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type number");
            return GameResult::LOSE;
        }
    };
    return guess(user_guess, secret_number);
}

// メイン関数
fn main() {
    println!("Guess the number!");
    let mut guess: String = String::new();
    let secret_number = gen_random_num(1..=100);
    loop {
        print!(">> ");
        // printマクロはすぐに反映されない
        // flush関数で標準出力に直ぐに表示させるようにしている
        // これを行わなければ、それより先にユーザ入力の受付処理が開始されてしまう
        stdout().flush().expect("Failed to reload stdout");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        match handle_input(guess.clone(), secret_number) {
            GameResult::WIN => {
                break;
            }
            GameResult::LOSE => {}
            GameResult::QUIT => {
                break;
            }
        }
        // read_line関数でユーザ入力を読み込む際、バッファに対し値をスタックのように積み上げていくので
        // 次のループに行く前に、前回のguessを消しておく必要がある
        guess.clear();
    }
}
