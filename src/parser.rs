use crate::tokenizer::{Token, TokenType};
use crate::util::{Skill, Resource, Shard, Teleporter};
use std::collections::HashSet;

pub enum ParseError {
    WrongToken(String, usize),
    WrongAmount(String, usize),
    WrongRequirement(String, usize),
    ParseInt(String, usize),
}

pub enum Requirement {
    Free,
    Definition(String),
    Pathset(String),
    Skill(Skill),
    EnergySkill(Skill, u16),
    Resource(Resource, u16),
    Shard(Shard),
    Teleporter(Teleporter),
    State(String),
    Quest(String),
    Damage(u16),
    Danger(u16),
    Combat(String),
    Boss(u16),
    BreakWall(u16),
    ShurikenBreak(u16),
    SentryJump(u16),
}
pub struct Line {
    pub ands: Vec<Requirement>,
    pub ors: Vec<Requirement>,
    pub group: Option<Group>,
}
pub struct Group {
    pub lines: Vec<Line>
}
pub struct Pathset {
    pub identifier: String,
    pub description: String,
}
pub struct Pathsets {
    pub identifier: String,
    pub pathsets: Vec<Pathset>,
}
pub enum RefillType {
    Full,
    Checkpoint,
    Health(u16),
    Energy(u16),
}
pub struct Refill {
    pub name: RefillType,
    pub requirements: Option<Group>,
}
pub enum ConnectionType {
    State,
    Quest,
    Pickup,
    Anchor,
}
pub struct Connection {
    pub name: ConnectionType,
    pub identifier: String,
    pub requirements: Option<Group>,
}
pub struct Definition {
    pub identifier: String,
    pub requirements: Group,
}
pub struct Region {
    pub identifier: String,
    pub requirements: Group,
}
pub struct Anchor {
    pub identifier: String,
    pub position: Option<(i16, i16)>,
    pub refills: Vec<Refill>,
    pub connections: Vec<Connection>,
}
pub struct Areas {
    pub definitions: Vec<Definition>,
    pub regions: Vec<Region>,
    pub anchors: Vec<Anchor>,
}

struct ParseContext {
    position: usize,
    definitions: HashSet<String>,
    pathsets: HashSet<String>,
    quests: HashSet<String>,
    states: HashSet<String>,
}

fn eat(tokens: &[Token], context: &mut ParseContext, expected_token_type: TokenType) -> Result<bool, ParseError> {
    let token_type = tokens[context.position].name;
    return if token_type == expected_token_type {
        context.position += 1;
        Ok(true)
    } else {
        Err(wrong_token(&tokens[context.position], &format!("{:?}", expected_token_type)))
    }
}

fn parse_requirement(token: &Token, context: &mut ParseContext) -> Result<Requirement, ParseError> {
    let mut parts = token.value.split('=');
    let keyword = parts.next().unwrap();
    let amount = parts.next();
    if parts.next().is_some() {
        return Err(wrong_amount(token));
    }
    match amount {
        Some(amount) => {
            if keyword == "Combat" {
                return Ok(Requirement::Combat(amount.to_string()));
            }
            let amount: u16 = match amount.parse() {
                Ok(result) => result,
                Err(_) => return Err(not_int(token)),
            };
            match keyword {
                "Blaze" => Ok(Requirement::EnergySkill(Skill::Blaze, amount)),
                "Boss" => Ok(Requirement::Boss(amount)),
                "Bow" => Ok(Requirement::EnergySkill(Skill::Bow, amount)),
                "BreakWall" => Ok(Requirement::BreakWall(amount)),
                "Damage" => Ok(Requirement::Damage(amount)),
                "Danger" => Ok(Requirement::Danger(amount)),
                "Energy" => Ok(Requirement::Resource(Resource::Energy, amount)),
                "Flash" => Ok(Requirement::EnergySkill(Skill::Flash, amount)),
                "Grenade" => Ok(Requirement::EnergySkill(Skill::Grenade, amount)),
                "Health" => Ok(Requirement::Resource(Resource::Health, amount)),
                "Keystone" => Ok(Requirement::Resource(Resource::Keystone, amount)),
                "Ore" => Ok(Requirement::Resource(Resource::Ore, amount)),
                "Sentry" => Ok(Requirement::EnergySkill(Skill::Sentry, amount)),
                "SentryJump" => Ok(Requirement::SentryJump(amount)),
                "ShardSlot" => Ok(Requirement::Resource(Resource::ShardSlot, amount)),
                "Shuriken" => Ok(Requirement::EnergySkill(Skill::Shuriken, amount)),
                "ShurikenBreak" => Ok(Requirement::ShurikenBreak(amount)),
                "Spear" => Ok(Requirement::EnergySkill(Skill::Spear, amount)),
                "SpiritLight" => Ok(Requirement::Resource(Resource::SpiritLight, amount)),
                _ => Err(wrong_requirement(token))
            }
        }
        None => match keyword {
            "Arcing" => Ok(Requirement::Shard(Shard::Arcing)),
            "Bash" => Ok(Requirement::Skill(Skill::Bash)),
            "Blaze" => Ok(Requirement::Skill(Skill::Blaze)),
            "Bow" => Ok(Requirement::Skill(Skill::Bow)),
            "Burrow" => Ok(Requirement::Skill(Skill::Burrow)),
            "BurrowsTP" => Ok(Requirement::Teleporter(Teleporter::Burrows)),
            "Catalyst" => Ok(Requirement::Shard(Shard::Catalyst)),
            "Dash" => Ok(Requirement::Skill(Skill::Dash)),
            "Deflector" => Ok(Requirement::Shard(Shard::Deflector)),
            "DenTP" => Ok(Requirement::Teleporter(Teleporter::Den)),
            "DepthsTP" => Ok(Requirement::Teleporter(Teleporter::Depths)),
            "DoubleJump" => Ok(Requirement::Skill(Skill::DoubleJump)),
            "EastPoolsTP" => Ok(Requirement::Teleporter(Teleporter::EastLuma)),
            "EastWastesTP" => Ok(Requirement::Teleporter(Teleporter::EastWastes)),
            "EastWoodsTP" => Ok(Requirement::Teleporter(Teleporter::EastWoods)),
            "EnergyHarvest" => Ok(Requirement::Shard(Shard::EnergyHarvest)),
            "Flap" => Ok(Requirement::Skill(Skill::Flap)),
            "Flash" => Ok(Requirement::Skill(Skill::Flash)),
            "Fracture" => Ok(Requirement::Shard(Shard::Fracture)),
            "free" => Ok(Requirement::Free),
            "GladesTP" => Ok(Requirement::Teleporter(Teleporter::Glades)),
            "Glide" => Ok(Requirement::Skill(Skill::Glide)),
            "Grapple" => Ok(Requirement::Skill(Skill::Grapple)),
            "Grenade" => Ok(Requirement::Skill(Skill::Grenade)),
            "Hammer" => Ok(Requirement::Skill(Skill::Hammer)),
            "HollowTP" => Ok(Requirement::Teleporter(Teleporter::Hollow)),
            "InnerRuinsTP" => Ok(Requirement::Teleporter(Teleporter::InnerRuins)),
            "Launch" => Ok(Requirement::Skill(Skill::Launch)),
            "LifeHarvest" => Ok(Requirement::Shard(Shard::LifeHarvest)),
            "Magnet" => Ok(Requirement::Shard(Shard::Magnet)),
            "MarshTP" => Ok(Requirement::Teleporter(Teleporter::Marsh)),
            "OuterRuinsTP" => Ok(Requirement::Teleporter(Teleporter::OuterRuins)),
            "Overflow" => Ok(Requirement::Shard(Shard::Overflow)),
            "ReachTP" => Ok(Requirement::Teleporter(Teleporter::Reach)),
            "Regenerate" => Ok(Requirement::Skill(Skill::Regenerate)),
            "Seir" => Ok(Requirement::Skill(Skill::Seir)),
            "Sentry" => Ok(Requirement::Skill(Skill::Sentry)),
            "ShriekTP" => Ok(Requirement::Teleporter(Teleporter::Shriek)),
            "Shuriken" => Ok(Requirement::Skill(Skill::Shuriken)),
            "Spear" => Ok(Requirement::Skill(Skill::Spear)),
            "Sticky" => Ok(Requirement::Shard(Shard::Sticky)),
            "Sword" => Ok(Requirement::Skill(Skill::Sword)),
            "TripleJump" => Ok(Requirement::Shard(Shard::TripleJump)),
            "Thorn" => Ok(Requirement::Shard(Shard::Thorn)),
            "UltraBash" => Ok(Requirement::Shard(Shard::UltraBash)),
            "UltraGrapple" => Ok(Requirement::Shard(Shard::UltraGrapple)),
            "WallJump" => Ok(Requirement::Skill(Skill::WallJump)),
            "WaterBreath" => Ok(Requirement::Skill(Skill::WaterBreath)),
            "WaterDash" => Ok(Requirement::Skill(Skill::WaterDash)),
            "Water" => Ok(Requirement::Skill(Skill::Water)),
            "WellspringTP" => Ok(Requirement::Teleporter(Teleporter::Wellspring)),
            "WestPoolsTP" => Ok(Requirement::Teleporter(Teleporter::WestLuma)),
            "WestWastesTP" => Ok(Requirement::Teleporter(Teleporter::WestWastes)),
            "WestWoodsTP" => Ok(Requirement::Teleporter(Teleporter::WestWoods)),
            "WillowTP" => Ok(Requirement::Teleporter(Teleporter::Willow)),
            _ if context.definitions.contains(keyword) => Ok(Requirement::Definition(keyword.to_string())),
            _ if context.pathsets.contains(keyword) => Ok(Requirement::Pathset(keyword.to_string())),
            _ if context.states.contains(keyword) => Ok(Requirement::State(keyword.to_string())),
            _ if context.quests.contains(keyword) => Ok(Requirement::Quest(keyword.to_string())),
            "Boss" => Err(wrong_amount(token)),
            "BreakWall" => Err(wrong_amount(token)),
            "Damage" => Err(wrong_amount(token)),
            "Danger" => Err(wrong_amount(token)),
            "Energy" => Err(wrong_amount(token)),
            "Health" => Err(wrong_amount(token)),
            "Keystone" => Err(wrong_amount(token)),
            "Ore" => Err(wrong_amount(token)),
            "SentryJump" => Err(wrong_amount(token)),
            "ShardSlot" => Err(wrong_amount(token)),
            "ShurikenBreak" => Err(wrong_amount(token)),
            "SpiritLight" => Err(wrong_amount(token)),
            _ => Err(wrong_requirement(token))
        }
    }
}

fn parse_free(tokens: &[Token], context: &mut ParseContext) -> Result<(), ParseError> {
    context.position += 1;
    match tokens[context.position].name {
        TokenType::Newline => context.position += 1,
        TokenType::Dedent => {},
        _ => return Err(wrong_token(&tokens[context.position], "new line after inline 'free'")),
    }
    Ok(())
}

fn parse_line(tokens: &[Token], context: &mut ParseContext) -> Result<Line, ParseError> {
    let mut ands = Vec::<Requirement>::new();
    let mut ors = Vec::<Requirement>::new();
    let mut group = None;
    loop {
        let token = &tokens[context.position];
        match token.name {
            TokenType::Requirement => {
                context.position += 1;
                match tokens[context.position].name {
                    TokenType::And => {
                        context.position += 1;
                        ands.push(parse_requirement(token, context)?);
                    },
                    TokenType::Or => {
                        context.position += 1;
                        ors.push(parse_requirement(token, context)?);
                    },
                    TokenType::Newline => {
                        context.position += 1;
                        if ors.is_empty() {
                            ands.push(parse_requirement(token, context)?);
                        } else {
                            ors.push(parse_requirement(token, context)?);
                        }
                        break;
                    },
                    TokenType::Dedent => {
                        if ors.is_empty() {
                            ands.push(parse_requirement(token, context)?);
                        } else {
                            ors.push(parse_requirement(token, context)?);
                        }
                        break;
                    },
                    TokenType::Group => {
                        context.position += 1;
                        ands.push(parse_requirement(token, context)?);
                        if let TokenType::Indent = tokens[context.position].name {
                            context.position += 1;
                            group = Some(parse_group(tokens, context)?);
                            break;
                        }
                    },
                    _ => return Err(wrong_token(token, "separator or end of line")),
                }
            }
            TokenType::Free => {
                parse_free(tokens, context)?;
                break;
            },
            _ => return Err(wrong_token(token, "requirement")),
        }
    }
    Ok(Line {
        ands,
        ors,
        group,
    })
}

fn parse_group(tokens: &[Token], context: &mut ParseContext) -> Result<Group, ParseError> {
    let mut lines = Vec::<Line>::new();
    loop {
        match tokens[context.position].name {
            TokenType::Requirement => lines.push(parse_line(tokens, context)?),
            TokenType::Dedent => break,
            _ => return Err(wrong_token(&tokens[context.position], "requirement or end of group")),
        }
    }
    // consume the dedent
    context.position += 1;
    Ok(Group {
        lines,
    })
}

fn parse_refill(tokens: &[Token], context: &mut ParseContext) -> Result<Refill, ParseError> {
    let identifier = &tokens[context.position].value;
    context.position += 1;

    let name;
    let mut requirements = None;
    match tokens[context.position].name {
        TokenType::Newline => context.position += 1,
        TokenType::Free => parse_free(tokens, context)?,
        TokenType::Indent => {
            context.position += 1;
            requirements = Some(parse_group(tokens, context)?)
        },
        _ => return Err(wrong_token(&tokens[context.position], "requirements or end of line")),
    }

    if identifier == "Checkpoint" {
        name = RefillType::Checkpoint;
    } else if identifier == "Full" {
        name = RefillType::Full;
    } else if let Some(amount) = identifier.strip_prefix("Health=") {
        let amount: u16 = match amount.parse() {
            Ok(result) => result,
            Err(_) => return Err(not_int(&tokens[context.position - 1])),
        };
        name = RefillType::Health(amount);
    } else if identifier == "Health" {
        name = RefillType::Health(1);
    } else if let Some(amount) = identifier.strip_prefix("Energy=") {
        let amount: u16 = match amount.parse() {
            Ok(result) => result,
            Err(_) => return Err(not_int(&tokens[context.position - 1])),
        };
        name = RefillType::Energy(amount);
    } else {
        return Err(wrong_token(&tokens[context.position], "'Checkpoint', 'Full', 'Health' or 'Energy'"));
    }

    Ok(Refill {
        name,
        requirements,
    })
}
fn parse_connection(tokens: &[Token], context: &mut ParseContext, name: ConnectionType) -> Result<Connection, ParseError> {
    let identifier = &tokens[context.position].value;
    let mut requirements = None;

    context.position += 1;
    match tokens[context.position].name {
        TokenType::Indent => {
            context.position += 1;
            requirements = Some(parse_group(tokens, context)?)
        },
        TokenType::Free => parse_free(tokens, context)?,
        _ => return Err(wrong_token(&tokens[context.position], "indent or 'free'")),
    }
    Ok(Connection {
        name,
        identifier: identifier.clone(),
        requirements,
    })
}
fn parse_state(tokens: &[Token], context: &mut ParseContext) -> Result<Connection, ParseError> {
    parse_connection(tokens, context, ConnectionType::State)
}
fn parse_quest(tokens: &[Token], context: &mut ParseContext) -> Result<Connection, ParseError> {
    parse_connection(tokens, context, ConnectionType::Quest)
}
fn parse_pickup(tokens: &[Token], context: &mut ParseContext) -> Result<Connection, ParseError> {
    parse_connection(tokens, context, ConnectionType::Pickup)
}
fn parse_anchor_connection(tokens: &[Token], context: &mut ParseContext) -> Result<Connection, ParseError> {
    parse_connection(tokens, context, ConnectionType::Anchor)
}
fn parse_pathset(tokens: &[Token], context: &mut ParseContext) -> Result<Pathset, ParseError> {
    let identifier = tokens[context.position].value.clone();
    let mut description = String::new();
    context.position += 1;
    let test = &tokens[context.position].name;
    if eat(tokens, context, TokenType::Group).is_ok() {
        eat(tokens, context, TokenType::Indent)?;
        loop {
            match tokens[context.position].name {
                TokenType::Requirement => {
                    if !description.is_empty() {
                        description += "\n";
                    }

                    description += &tokens[context.position].value;
                    context.position += 1;
                },
                TokenType::Dedent => break,
                _ => return Err(wrong_token(&tokens[context.position], "pathset entry")),
            }
        }

        // consume the dedent
        context.position += 1;
    } else {
        // Try to eat a newline, we don't care if we fail.
        let _ = eat(tokens, context, TokenType::Newline);
    }

    Ok(Pathset {
        identifier,
        description
    })
}
fn parse_pathsets(tokens: &[Token], context: &mut ParseContext) -> Result<Pathsets, ParseError> {
    let identifier = tokens[context.position].value.clone();
    let mut pathsets = Vec::new();
    context.position += 1;
    eat(tokens, context, TokenType::Indent)?;
    loop {
        match tokens[context.position].name {
            TokenType::Requirement => {
                pathsets.push(parse_pathset(tokens, context)?);
            },
            TokenType::Dedent => break,
            _ => return Err(wrong_token(&tokens[context.position], "requirement or end of group")),
        }
    }

    eat(tokens, context, TokenType::Dedent)?;
    return if pathsets.is_empty() {
        Err(wrong_token(&tokens[context.position], "pathset entry"))
    } else {
        Ok(Pathsets {
            identifier,
            pathsets,
        })
    }
}
fn parse_named_group(tokens: &[Token], context: &mut ParseContext) -> Result<(String, Group), ParseError> {
    let identifier = &tokens[context.position].value;
    let requirements;
    context.position += 1;
    match tokens[context.position].name {
        TokenType::Indent => {
            context.position += 1;
            requirements = parse_group(tokens, context)?;
        },
        _ => return Err(wrong_token(&tokens[context.position], "indent")),
    }

    Ok((
        identifier.clone(),
        requirements,
    ))
}

fn parse_region(tokens: &[Token], context: &mut ParseContext) -> Result<Region, ParseError> {
    let (identifier, requirements) = parse_named_group(tokens, context)?;
    Ok(Region {
        identifier,
        requirements,
    })
}
fn parse_definition(tokens: &[Token], context: &mut ParseContext) -> Result<Definition, ParseError> {
    let (identifier, requirements) = parse_named_group(tokens, context)?;
    Ok(Definition {
        identifier,
        requirements,
    })
}
fn parse_anchor(tokens: &[Token], context: &mut ParseContext) -> Result<Anchor, ParseError> {
    let identifier = &tokens[context.position].value;
    let mut position = None;
    context.position += 1;
    {
        let token = &tokens[context.position];
        if let TokenType::Position = token.name {
            let mut coords = token.value.split(',');
            let x: i16 = match coords.next().unwrap().parse() {
                Ok(result) => result,
                Err(_) => return Err(not_int(token)),
            };
            let y: i16 = match coords.next().unwrap().parse() {
                Ok(result) => result,
                Err(_) => return Err(not_int(token)),
            };
            position = Some((x, y));
            context.position += 1;
        }
    }

    let mut refills = Vec::<Refill>::new();
    let mut connections = Vec::<Connection>::new();

    match tokens[context.position].name {
        TokenType::Indent => {
            context.position += 1;
            loop {
                match tokens[context.position].name {
                    TokenType::Refill => refills.push(parse_refill(tokens, context)?),
                    TokenType::State => connections.push(parse_state(tokens, context)?),
                    TokenType::Quest => connections.push(parse_quest(tokens, context)?),
                    TokenType::Pickup => connections.push(parse_pickup(tokens, context)?),
                    TokenType::Connection => connections.push(parse_anchor_connection(tokens, context)?),
                    TokenType::Dedent => {
                        context.position += 1;
                        break;
                    },
                    _ => return Err(wrong_token(&tokens[context.position], "refill, state, quest, pickup, connection or end of anchor")),
                }
            }
        },
        _ => return Err(wrong_token(&tokens[context.position], "indent")),
    }
    Ok(Anchor {
        identifier: identifier.to_string(),
        position,
        refills,
        connections,
    })
}

fn wrong_token(token: &Token, description: &str) -> ParseError {
    ParseError::WrongToken(format!("Expected {} at line {}, instead found {:?}", description, token.line, token.name), token.position)
}
fn wrong_amount(token: &Token) -> ParseError {
    ParseError::WrongAmount(format!("Failed to parse amount at line {}", token.line), token.position)
}
fn wrong_requirement(token: &Token) -> ParseError {
    ParseError::WrongRequirement(format!("Failed to parse requirement at line {}", token.line), token.position)
}
fn not_int(token: &Token) -> ParseError {
    ParseError::ParseInt(format!("Need an integer in {:?} at line {}", token.name, token.line), token.position)
}

fn preprocess(tokens: &[Token], context: &mut ParseContext) -> Result<bool, ParseError> {
    // Find all states so we can differentiate states from pathsets.
    let end = tokens.len();
    while context.position < end {
        let token = &tokens[context.position];
        match token.name {
            TokenType::Definition => { context.definitions.insert(token.value.clone()); },
            TokenType::Pathsets => {
                let pathsets = parse_pathsets(tokens, context)?;
                context.pathsets.extend(
                pathsets.pathsets
                .iter()
                .map(|pathset| pathset.identifier.clone())
                );
                context.position -= 1;
            },
            TokenType::Quest => { context.quests.insert(token.value.clone()); },
            TokenType::State => { context.states.insert(token.value.clone()); },
            _ => {},
        }

        context.position += 1;
    }

    Ok(true)
}

fn process(tokens: &[Token], context: &mut ParseContext) -> Result<Areas, ParseError> {
    let end = tokens.len();
    let mut definitions = Vec::<Definition>::new();
    let mut regions = Vec::<Region>::new();
    let mut anchors = Vec::<Anchor>::new();

    if let TokenType::Newline = tokens[context.position].name { context.position += 1 }

    while context.position < end {
        match tokens[context.position].name {
            // We have already parsed the pathsets in the preprocess step so just eat here.
            TokenType::Pathsets => { parse_pathsets(tokens, context)?; },
            TokenType::Definition => { definitions.push(parse_definition(tokens, context)?); },
            TokenType::Region => { regions.push(parse_region(tokens, context)?); },
            TokenType::Anchor => { anchors.push(parse_anchor(tokens, context)?); },
            _ => { return Err(wrong_token(&tokens[context.position], "definition or anchor")); },
        }
    }
    Ok(Areas {
        definitions,
        regions,
        anchors,
    })
}

pub fn parse_areas(tokens: &[Token]) -> Result<Areas, ParseError> {
    let mut context = ParseContext {
        position: 0,
        definitions: Default::default(),
        pathsets: Default::default(),
        quests: Default::default(),
        states: Default::default(),
    };

    preprocess(tokens, &mut context)?;
    context.position = 0;
    return process(tokens, &mut context);
}