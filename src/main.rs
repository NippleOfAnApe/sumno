#![feature(string_remove_matches)]
use colored::*;
use scraper::{Html, Selector};

fn main() {
    // Завантажити сторінку у бафер
    // let response = reqwest::blocking::get("http://sum.in.ua/s/vidkryvaty")  // Багато означень, 0 іллюстрацій
    // let response = reqwest::blocking::get("http://sum.in.ua/random")
    let response = reqwest::blocking::get("http://sum.in.ua/s/TURA")
    // Якщо неможливо завантажити сторінку - паніка із повідомленням
        .expect("Couldn't load a webpage.").text().unwrap();
    let document = Html::parse_document(&response);

    // Обрати лише тіло статті без нижнього колонотитула
    let article_body_selector = scraper::Selector::parse(r#"div[itemprop="articleBody"]"#).unwrap();
    let fragment = document.select(&article_body_selector).next().unwrap();

    // Визначити селектори за якими обирати html теги
    let p_selector = Selector::parse("p").unwrap();
    let strong_title_selector = Selector::parse("strong.title").unwrap();
    let illus_selector = Selector::parse("i.illus").unwrap();

    let znach_delimeter = String::from("****").cyan();
    let illus_delimeter = String::from("----").blue();

    // Якщр назва та визначення знаходяться на окремих рядках
    if fragment.select(&p_selector).count() > 1
    {
        for (i, element) in fragment.select(&p_selector).enumerate() {
            // Перший елемент - саме слово
            if i < 1
            {
                for title in element.select(&strong_title_selector)
                {
                    let word = title.text().collect::<String>().bright_green().bold();
                    print!("{}\t", word);
                }
                println!("\n");
            }
            // Означення
            else
            {
                // Зберегти усі приклади як окремі елементи у масиві
                let mut illustrations: Vec<String> = vec![];
                for child in element.select(&illus_selector)
                {
                    let word = child.text().collect::<String>();
                    illustrations.push(word);
                }

                // Якщо немає прикладів - то вони у тому ж параграфі що і означення. Тож ліпше роздрукувати його монотонно.
                if illustrations.len() != 0
                {
                    // Зберегти означення у окремому масиві щоб роздрукувати їх та приклади використовуючи різні стилі
                    let mut znachennya: Vec<String> = vec![];
                    for znach in element.children() {
                        // Ототримати доступ до необробленого тексту
                        if let scraper::Node::Text(t) = znach.value()
                        {
                            let new_text = &t.text.replace(";", "");
                            // ігнорувати обрізані слова
                            if t.len() > 15 {znachennya.push(new_text.to_string())}
                        }
                    }

                    // Охайний друк
                    println!("{}", znach_delimeter);
                    for znachen in znachennya.iter()
                    {
                        println!("{}\t", znachen.bold());
                    }
                    println!("{}", illus_delimeter);
                    for il in illustrations.iter()
                    {
                        println!("{}\n", il.italic().yellow());
                    }
                }
                else
                {
                    // Друкувати повністю параграф
                    let cum = element.text().collect::<String>();
                    println!("{}", znach_delimeter);
                    println!("{}", cum);
                }
            }
        }
    }
    // Слово, означення та приклади у одному параграфі
    else
    {
        // Щоб не ітерувати через масив із одним елементом
        let cum = fragment.select(&p_selector).last().unwrap();

        // Друк слова
        for title in cum.select(&strong_title_selector)
        {
            let word = title.text().collect::<String>().green().bold();
            print!("{}\t", word);
        }
        println!("\n");

        // Один параграф означає що може бути лише одне означення слова, тож зберігаємо означення одним рядком
        let mut p_text = String::new();
        for child in cum.children()
        {
            if let scraper::Node::Text(t) = child.value()
            {
                // Укоротити від оскорочених слів
                if t.len() > 15 {p_text.push_str(&t.text)}
            }
        }

        // Якщо немає означення то це посилання на інше слово, тож друкуєм параграф повністю + приклади
        if p_text.is_empty()
        {
            let mut cum = cum.text().collect::<String>();
            let mut exaples: Vec<String> = vec![];
            for illus in fragment.select(&illus_selector)
            {
                // Видалити кожний приклад у окремий масив
                let pryklad = illus.text().collect::<String>();
                cum.remove_matches(&pryklad);
                exaples.push(pryklad);
            }

            // Охайний друк із прикладами
            println!("{}", znach_delimeter);
            println!("{}", cum);
            println!("{}", illus_delimeter);
            for item in exaples.iter()
            {
                println!("{}\n", item.italic().yellow());
            }
        }
        else
        {
            // Охайний друк із прикладами
            println!("{}", znach_delimeter);
            println!("{}", p_text.bold());
            println!("{}", illus_delimeter);

            for illus in fragment.select(&illus_selector)
            {
                let pryklad = illus.text().collect::<String>();
                println!("{}\n", pryklad.italic().yellow());
            }
        }

    }
}
