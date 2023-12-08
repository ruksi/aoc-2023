pub fn solve_day07_part1(input: String) -> Result<String, String> {
    let mut hands = Hand::many_from(&input, None)?;
    hands.sort();
    let winnings = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum::<usize>();
    Ok(winnings.to_string())
}

pub fn solve_day07_part2(input: String) -> Result<String, String> {
    let mut hands = Hand::many_from(&input, Some('J'))?;
    hands.sort();
    let winnings = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum::<usize>();
    Ok(winnings.to_string())
}

#[derive(Eq)]
struct Hand {
    cards: Vec<u8>,
    bid: usize,
}

impl Hand {
    fn from(input: &str, joker: Option<char>) -> Result<Hand, String> {
        let mut parts = input.split_whitespace();
        let cards_str = parts.next().ok_or("No cards on line")?;
        let joker_char = joker.unwrap_or('?');
        let cards = cards_str
            .chars()
            .map(|c| match c {
                _ if c == joker_char => JOKER_INDEX,  // joker is the lowest >:(
                _ if c.to_digit(10)
                    .and_then(|n| if n >= 2 && n <= 9 { Some(n) } else { None })
                    .is_some() => c.to_digit(10).unwrap() as u8,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("Invalid card '{}'", c),
            })
            .collect::<Vec<_>>();
        let bid_str = parts.next().ok_or("No bid on line")?;
        let bid = bid_str.parse::<usize>().map_err(|e| e.to_string())?;
        Ok(Hand { cards, bid })
    }

    fn many_from(input: &str, joker: Option<char>) -> Result<Vec<Hand>, String> {
        let hands = input
            .lines()
            .map(|line| Hand::from(line, joker))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(hands)
    }

    fn primary_score(&self) -> u8 {
        let mut counts = [0; 15];
        for card in self.cards.iter() {
            counts[*card as usize] += 1;
        }

        // throw away cards on the joker index, they are used as wildcards
        let jokers = counts[JOKER_INDEX as usize];
        counts[JOKER_INDEX as usize] = 0;

        if counts.iter().find(|c| **c + jokers >= 5).is_some() { return FIVE_OF_A_KIND; }
        if counts.iter().find(|c| **c + jokers >= 4).is_some() { return FOUR_OF_A_KIND; }

        // if we had more than 2 jokers, one of the checks above would have matched already
        assert!(jokers <= 2);

        // if we had 2 of anything to form a full house, we would have matched something better
        if jokers == 2 { return THREE_OF_A_KIND; }

        let threes_count = counts.iter().filter(|c| **c == 3).count();
        let pair_count = counts.iter().filter(|c| **c == 2).count();
        if threes_count == 1 {
            if pair_count == 1 { return FULL_HOUSE; }
            return THREE_OF_A_KIND;
        }
        if pair_count == 2 {
            if jokers == 1 { return FULL_HOUSE; }
            return TWO_PAIRS;
        }
        if pair_count == 1 {
            if jokers == 1 { return THREE_OF_A_KIND; }
            return ONE_PAIR;
        }
        if jokers == 1 { return ONE_PAIR; }
        HIGH_CARD
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_score = self.primary_score();
        let other_score = other.primary_score();
        if self_score != other_score {
            return self_score.cmp(&other_score);
        }

        // secondary scoring by card values
        let card_pairs = self.cards.iter().zip(other.cards.iter());
        for (self_card, other_card) in card_pairs {
            if self_card != other_card {
                return self_card.cmp(other_card);
            }
        }

        std::cmp::Ordering::Equal
    }
}

impl PartialOrd<Hand> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<Hand> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

// 1 is reserved for the joker~ 🃏
const JOKER_INDEX: u8 = 1;

const FIVE_OF_A_KIND: u8 = 7;
const FOUR_OF_A_KIND: u8 = 6;
const FULL_HOUSE: u8 = 5;
const THREE_OF_A_KIND: u8 = 4;
const TWO_PAIRS: u8 = 3;
const ONE_PAIR: u8 = 2;
const HIGH_CARD: u8 = 1;


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn primary_scoring() -> Result<(), String> {
        assert_eq!(Hand::from("AAAAA 0", None)?.primary_score(), FIVE_OF_A_KIND);
        assert_eq!(Hand::from("AAAAK 0", None)?.primary_score(), FOUR_OF_A_KIND);
        assert_eq!(Hand::from("AAAKK 0", None)?.primary_score(), FULL_HOUSE);
        assert_eq!(Hand::from("AAAKQ 0", None)?.primary_score(), THREE_OF_A_KIND);
        assert_eq!(Hand::from("AAKKQ 0", None)?.primary_score(), TWO_PAIRS);
        assert_eq!(Hand::from("AAKQJ 0", None)?.primary_score(), ONE_PAIR);
        assert_eq!(Hand::from("23456 0", None)?.primary_score(), HIGH_CARD);
        Ok(())
    }

    #[test]
    fn primary_scoring_with_jokers() -> Result<(), String> {
        assert_eq!(Hand::from("32T3K 0", Some('J'))?.primary_score(), ONE_PAIR);
        assert_eq!(Hand::from("KK677 0", Some('J'))?.primary_score(), TWO_PAIRS);
        assert_eq!(Hand::from("24567 0", Some('J'))?.primary_score(), HIGH_CARD);
        assert_eq!(Hand::from("JJJJJ 0", Some('J'))?.primary_score(), FIVE_OF_A_KIND);
        assert_eq!(Hand::from("JJJJA 0", Some('J'))?.primary_score(), FIVE_OF_A_KIND);
        assert_eq!(Hand::from("JJJAA 0", Some('J'))?.primary_score(), FIVE_OF_A_KIND);
        assert_eq!(Hand::from("JJAAA 0", Some('J'))?.primary_score(), FIVE_OF_A_KIND);
        assert_eq!(Hand::from("JAAAA 0", Some('J'))?.primary_score(), FIVE_OF_A_KIND);
        assert_eq!(Hand::from("KTJJJ 0", Some('J'))?.primary_score(), FOUR_OF_A_KIND);
        assert_eq!(Hand::from("KTTJJ 0", Some('J'))?.primary_score(), FOUR_OF_A_KIND);
        assert_eq!(Hand::from("KTTTJ 0", Some('J'))?.primary_score(), FOUR_OF_A_KIND);
        assert_eq!(Hand::from("KQTJJ 0", Some('J'))?.primary_score(), THREE_OF_A_KIND);
        assert_eq!(Hand::from("KQTTJ 0", Some('J'))?.primary_score(), THREE_OF_A_KIND);
        assert_eq!(Hand::from("AKQJT 0", Some('J'))?.primary_score(), ONE_PAIR);
        Ok(())
    }

    #[test]
    fn ordering() -> Result<(), String> {
        assert!(Hand::from("AAAAA 0", None)? > Hand::from("KKKKK 0", None)?);
        assert!(Hand::from("KKKKK 0", None)? > Hand::from("AAAAK 0", None)?);
        assert!(Hand::from("KKKKK 0", None)? == Hand::from("KKKKK 9", None)?);
        assert!(Hand::from("KKAAQ 0", None)? < Hand::from("QQQ23 0", None)?);
        Ok(())
    }

    #[test]
    fn ordering_with_jokers() -> Result<(), String> {
        assert!(Hand::from("QQQQ2 0", Some('J'))? < Hand::from("KKKJ2 0", Some('J'))?);
        assert!(Hand::from("QQQQ2 0", Some('J'))? > Hand::from("JKKK2 0", Some('J'))?);
        assert!(Hand::from("TTTT2 0", Some('J'))? > Hand::from("JKKK2 0", Some('J'))?);
        Ok(())
    }

    #[test]
    fn solve_day07_part1_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day07_part1(input)?, "6440");
        Ok(())
    }

    #[test]
    fn solve_day07_part1_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day07_part1(input)?, "250120186");
        Ok(())
    }

    #[test]
    fn solve_day07_part2_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day07_part2(input)?, "5905");
        Ok(())
    }

    #[test]
    fn solve_day07_part2_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day07_part2(input)?, "250665248");
        Ok(())
    }
}
