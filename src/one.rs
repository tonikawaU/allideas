use rand::Rng;
// этот трейт был добавлен через Cargo.toml, потому, задать путь как это есть с std не получиться

pub fn guessing() {
    println!("time to guess!");

    let intended_number: u32 = rand::thread_rng().gen_range(1..=100);

    // thread_rng() // функция которая создает и возвращает генератор рандомных чисел, после возвращения память в оперативе освобождаеться(дропается).
    // thread_rng() // функция из rand, а gen_range лишь метод его применения взятый из rand::Rng.
    // gen_range() // принимает за аргумент диапазон находящийся в скобках, часть rand::Rng.
    // gen_range() // = в скобках позволяет gen_range вносить в диапазон также 100, было бы (1..100) то диапазон был бы от 1 до 99, это база для всего языка, не только gen_range.

    println!("intended one is: {intended_number}.");

    let mut guess: String = String::new();

    // создание пустой мутабельной переменной guess

    std::io::stdin()
        .read_line(&mut guess)
        // std::io::stdin это коридор для возврата(получения по запросу) обьекта Stdin, уже у которого есть метод read_line() который на следующей строке и вызван.
        // & это ссылка на переменную guess, а mut перед ее названием дает право read_line изменять ее содержание.
        // read_line это по сути аналог input() из питончега.
        .expect("failed to read.");
    // .expect в данном случае просто обработчик Result, помимо него также .expect может обрабатывать другой enum, Option который отвечает за присутствие или отсутствие элемента где либо, где оно не обязательно.
    // если enum Result из 6 строки вернет одним из своих вариантов Err, то прога крашнеться с сообщением в кавычках, если с вариантом Ok, вернет результат 6 строки.
    // Проще говоря он тут нужен для корректной работы, где при результате любом кроме Ok, он спецом крашит прогу что бы дать сигнал об некорректной работе.

    let guess: u32 = guess.trim().parse().expect("type a number");

    // с помощью shadowing переменная guess приобретает тип данных u32, оставляя то же значение.

    println!("input the number you think is intended.");

    match guess.cmp(&intended_number) {
        std::cmp::Ordering::Less => println!("too small."),
        std::cmp::Ordering::Greater => println!("too big."),
        std::cmp::Ordering::Equal => println!("you win."),
    }

    // cmp - compare, этот метод сравнвает переменную со значением в скобках, а далее enum Ordering возвращает сравниваемое значение и делает то, что указано в "=>"
    //

    println!("you hit: {guess}.");

    // вывод юзеру его же ввод.
}
