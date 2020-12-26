use nom::IResult;
use std::io::prelude::*;

pub fn parse_input<R: Read>(r: &mut R) -> (Vec<Field>, Ticket, Vec<Ticket>) {
    let input = {
        let mut buf = String::new();
        r.read_to_string(&mut buf).unwrap();
        buf
    };

    let mut sections = input.split("\n\n");

    let (_, fields) = parse_fields(sections.next().unwrap()).unwrap();
    let (_, (_, mut my_ticket)) = parse_tickets(sections.next().unwrap()).unwrap();
    let (_, (_, nearby_tickets)) = parse_tickets(sections.next().unwrap()).unwrap();

    (fields, my_ticket.pop().unwrap(), nearby_tickets)
}

pub fn part1(fields: &[Field], tickets: &[Ticket]) -> usize {
    tickets
        .iter()
        .flatten()
        .filter(|v| fields.iter().all(|f| !f.valid_value(v)))
        .sum()
}

pub fn part2(fields: &[Field], tickets: &[Ticket], my_ticket: &Ticket) -> usize {
    use std::collections::HashMap;

    let valid_tickets: Vec<Ticket> = tickets
        .iter()
        .cloned()
        .filter(|t| t.iter().all(|v| !fields.iter().all(|f| !f.valid_value(v))))
        .collect();

    let n_columns = valid_tickets[0].len();
    let candidates: Vec<Vec<String>> = (0..n_columns)
        .into_iter()
        .map(|i| {
            let values: Vec<TicketV> = valid_tickets.iter().map(|t| t[i]).collect();
            fields
                .iter()
                .filter_map(|f| {
                    if values.iter().all(|v| f.valid_value(v)) {
                        Some(f.name.clone())
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();

    // field -> column
    let mut field_map: HashMap<String, usize> = HashMap::new();
    loop {
        let f = candidates.iter().enumerate().find(|(_colnum, fields)| {
            fields
                .iter()
                .filter(|fname| !field_map.contains_key(fname.as_str()))
                .count()
                == 1
        });
        if let Some((colnum, fields)) = f {
            let field_name: String = fields
                .iter()
                .filter(|fname| !field_map.contains_key(fname.as_str()))
                .next()
                .unwrap()
                .to_string();
            field_map.insert(field_name, colnum);
        } else {
            break;
        }
    }

    // dbg!(&field_map);

    field_map
        .iter()
        .filter(|(field_name, _column)| field_name.starts_with("departure"))
        .map(|(_field_name, &column)| my_ticket[column])
        .product()
    // todo!();
}

#[derive(Debug)]
pub struct Range {
    low: usize,
    high: usize,
}

impl Range {
    fn contains(&self, x: &TicketV) -> bool {
        (self.low..=self.high).contains(x)
    }
}

#[derive(Debug)]
pub struct Field {
    name: String,
    ranges: Vec<Range>,
}

impl Field {
    fn valid_value(&self, x: &TicketV) -> bool {
        self.ranges.iter().any(|r| r.contains(x))
    }
}

pub type Ticket = Vec<TicketV>;
pub type TicketV = usize;

fn parse_range(i: &str) -> IResult<&str, Range> {
    use nom::character::complete::*;
    use nom::combinator::map_opt;

    let (i, low) = map_opt(digit1, |x: &str| x.parse::<usize>().ok())(i)?;
    let (i, _) = char('-')(i)?;
    let (i, high) = map_opt(digit1, |x: &str| x.parse::<usize>().ok())(i)?;

    Ok((i, Range { low, high }))
}

fn parse_field(i: &str) -> IResult<&str, Field> {
    use nom::bytes::complete::{tag, take_until};

    let (i, name) = take_until(": ")(i)?;
    let (i, _) = tag(": ")(i)?;
    let (i, r1) = parse_range(i)?;
    let (i, _) = tag(" or ")(i)?;
    let (i, r2) = parse_range(i)?;

    Ok((
        i,
        Field {
            name: name.to_string(),
            ranges: vec![r1, r2],
        },
    ))
}

fn parse_tickets(i: &str) -> IResult<&str, (String, Vec<Ticket>)> {
    use nom::bytes::complete::{tag, take_until};
    use nom::character::complete::*;
    use nom::multi::separated_list1;

    let (i, field) = take_until(":\n")(i)?;
    let (i, _) = tag(":\n")(i)?;
    let (i, tickets) = separated_list1(newline, parse_ticket)(i)?;
    Ok((i, (field.to_string(), tickets)))
}

fn parse_ticket(i: &str) -> IResult<&str, Vec<TicketV>> {
    use nom::character::complete::*;
    use nom::combinator::map_opt;
    use nom::multi::separated_list1;

    let (i, values) = separated_list1(
        char(','),
        map_opt(digit1, |x: &str| x.parse::<usize>().ok()),
    )(i)?;
    Ok((i, values))
}

fn parse_fields(i: &str) -> IResult<&str, Vec<Field>> {
    use nom::character::complete::*;
    use nom::multi::separated_list1;

    separated_list1(newline, parse_field)(i)
}
