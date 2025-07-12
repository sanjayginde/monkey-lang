use std::fmt::Debug;
use std::marker::PhantomData;

// Example 1: Type-level state machines to avoid runtime polymorphism
// Instead of Box<dyn State>, we use PhantomData to encode state at compile time

#[derive(Debug)]
pub struct Locked;

#[derive(Debug)]
pub struct Unlocked;

#[derive(Debug)]
pub struct SafeBox<State> {
    contents: String,
    _state: PhantomData<State>,
}

impl SafeBox<Locked> {
    pub fn new(contents: String) -> Self {
        SafeBox {
            contents,
            _state: PhantomData,
        }
    }

    // Only locked boxes can be unlocked
    pub fn unlock(self, key: &str) -> Result<SafeBox<Unlocked>, Self> {
        if key == "secret" {
            Ok(SafeBox {
                contents: self.contents,
                _state: PhantomData,
            })
        } else {
            Err(self)
        }
    }
}

impl SafeBox<Unlocked> {
    // Only unlocked boxes can access contents
    pub fn get_contents(&self) -> &str {
        &self.contents
    }

    pub fn set_contents(&mut self, contents: String) {
        self.contents = contents;
    }

    // Only unlocked boxes can be locked
    pub fn lock(self) -> SafeBox<Locked> {
        SafeBox {
            contents: self.contents,
            _state: PhantomData,
        }
    }
}

// Example 2: Type-level programming for different parser modes
// Instead of Box<dyn Parser>, we encode the parsing strategy in the type

pub trait ParseStrategy {
    type Output;
    fn parse(input: &str) -> Self::Output;
}

#[derive(Debug)]
pub struct StrictMode;

#[derive(Debug)]
pub struct LenientMode;

impl ParseStrategy for StrictMode {
    type Output = Result<i32, String>;

    fn parse(input: &str) -> Self::Output {
        input.parse::<i32>().map_err(|e| e.to_string())
    }
}

impl ParseStrategy for LenientMode {
    type Output = i32;

    fn parse(input: &str) -> Self::Output {
        input
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap_or(0)
    }
}

#[derive(Debug)]
pub struct Parser<Strategy> {
    _strategy: PhantomData<Strategy>,
}

impl<Strategy: ParseStrategy> Parser<Strategy> {
    pub fn new() -> Self {
        Parser {
            _strategy: PhantomData,
        }
    }

    pub fn parse(&self, input: &str) -> Strategy::Output {
        Strategy::parse(input)
    }
}

// Example 3: Units of measurement with compile-time safety
// Instead of Box<dyn Unit>, we encode units in the type system

#[derive(Debug, Clone, Copy)]
pub struct Meters;

#[derive(Debug, Clone, Copy)]
pub struct Feet;

#[derive(Debug, Clone, Copy)]
pub struct Measurement<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

impl<Unit> Measurement<Unit> {
    pub fn new(value: f64) -> Self {
        Measurement {
            value,
            _unit: PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl Measurement<Meters> {
    pub fn to_feet(self) -> Measurement<Feet> {
        Measurement::new(self.value * 3.28084)
    }
}

impl Measurement<Feet> {
    pub fn to_meters(self) -> Measurement<Meters> {
        Measurement::new(self.value / 3.28084)
    }
}

// This prevents adding incompatible units at compile time
impl<Unit> std::ops::Add for Measurement<Unit> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Measurement::new(self.value + other.value)
    }
}

// Example 4: Database connection states
// Instead of Box<dyn Connection>, we use PhantomData for connection state

#[derive(Debug)]
pub struct Connected;

#[derive(Debug)]
pub struct Disconnected;

#[derive(Debug)]
pub struct Database<State> {
    connection_string: String,
    _state: PhantomData<State>,
}

impl Database<Disconnected> {
    pub fn new(connection_string: String) -> Self {
        Database {
            connection_string,
            _state: PhantomData,
        }
    }

    pub fn connect(self) -> Result<Database<Connected>, String> {
        println!("Connecting to: {}", self.connection_string);
        // Simulate connection logic
        Ok(Database {
            connection_string: self.connection_string,
            _state: PhantomData,
        })
    }
}

impl Database<Connected> {
    pub fn query(&self, sql: &str) -> Vec<String> {
        println!("Executing query: {}", sql);
        vec!["result1".to_string(), "result2".to_string()]
    }

    pub fn disconnect(self) -> Database<Disconnected> {
        println!("Disconnecting from database");
        Database {
            connection_string: self.connection_string,
            _state: PhantomData,
        }
    }
}

// Example 5: Type-level AST node marking
// Instead of Box<dyn ASTNode>, we can use PhantomData to mark node types

pub trait NodeType: Debug {}

#[derive(Debug)]
pub struct Expression;

#[derive(Debug)]
pub struct Statement;

impl NodeType for Expression {}
impl NodeType for Statement {}

#[derive(Debug)]
pub struct ASTNode<T: NodeType> {
    pub value: String,
    pub line: usize,
    pub column: usize,
    _node_type: PhantomData<T>,
}

impl<T: NodeType> ASTNode<T> {
    pub fn new(value: String, line: usize, column: usize) -> Self {
        ASTNode {
            value,
            line,
            column,
            _node_type: PhantomData,
        }
    }
}

// Type aliases for clarity
pub type ExpressionNode = ASTNode<Expression>;
pub type StatementNode = ASTNode<Statement>;

// We can have different behavior for different node types
impl ASTNode<Expression> {
    pub fn evaluate(&self) -> i32 {
        // Expression-specific evaluation logic
        42
    }
}

impl ASTNode<Statement> {
    pub fn execute(&self) {
        // Statement-specific execution logic
        println!("Executing statement: {}", self.value);
    }
}

// Example 6: Builder pattern with compile-time validation
// Instead of Box<dyn Builder>, we use PhantomData to track builder state

#[derive(Debug)]
pub struct NoName;

#[derive(Debug)]
pub struct HasName;

#[derive(Debug)]
pub struct NoAge;

#[derive(Debug)]
pub struct HasAge;

#[derive(Debug)]
pub struct PersonBuilder<NameState, AgeState> {
    name: Option<String>,
    age: Option<u32>,
    _name_state: PhantomData<NameState>,
    _age_state: PhantomData<AgeState>,
}

impl PersonBuilder<NoName, NoAge> {
    pub fn new() -> Self {
        PersonBuilder {
            name: None,
            age: None,
            _name_state: PhantomData,
            _age_state: PhantomData,
        }
    }
}

impl<AgeState> PersonBuilder<NoName, AgeState> {
    pub fn name(self, name: String) -> PersonBuilder<HasName, AgeState> {
        PersonBuilder {
            name: Some(name),
            age: self.age,
            _name_state: PhantomData,
            _age_state: PhantomData,
        }
    }
}

impl<NameState> PersonBuilder<NameState, NoAge> {
    pub fn age(self, age: u32) -> PersonBuilder<NameState, HasAge> {
        PersonBuilder {
            name: self.name,
            age: Some(age),
            _name_state: PhantomData,
            _age_state: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

// Only complete builders can build
impl PersonBuilder<HasName, HasAge> {
    pub fn build(self) -> Person {
        Person {
            name: self.name.unwrap(),
            age: self.age.unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_box_state_machine() {
        let locked_box = SafeBox::new("secret data".to_string());

        // This would fail to compile: locked_box.get_contents();

        let unlocked_box = locked_box.unlock("secret").unwrap();
        assert_eq!(unlocked_box.get_contents(), "secret data");

        let _locked_again = unlocked_box.lock();
    }

    #[test]
    fn test_parser_strategies() {
        let strict_parser = Parser::<StrictMode>::new();
        let lenient_parser = Parser::<LenientMode>::new();

        assert!(strict_parser.parse("abc").is_err());
        assert_eq!(strict_parser.parse("123").unwrap(), 123);

        assert_eq!(lenient_parser.parse("abc123def456"), 123456);
        assert_eq!(lenient_parser.parse("no digits"), 0);
    }

    #[test]
    fn test_measurement_units() {
        let distance_m = Measurement::<Meters>::new(10.0);
        let distance_ft = distance_m.to_feet();

        // This would fail to compile due to type mismatch:
        // let invalid = distance_m + distance_ft;

        let total_meters = distance_m + Measurement::<Meters>::new(5.0);
        assert_eq!(total_meters.value(), 15.0);
    }

    #[test]
    fn test_database_connection_states() {
        let db = Database::new("localhost:5432".to_string());

        // This would fail to compile: db.query("SELECT * FROM users");

        let connected_db = db.connect().unwrap();
        let results = connected_db.query("SELECT * FROM users");
        assert_eq!(results.len(), 2);

        let _disconnected_db = connected_db.disconnect();
    }

    #[test]
    fn test_ast_node_types() {
        let expr = ExpressionNode::new("x + y".to_string(), 1, 5);
        let stmt = StatementNode::new("let x = 5".to_string(), 2, 1);

        assert_eq!(expr.evaluate(), 42);
        stmt.execute();
    }

    #[test]
    fn test_builder_pattern() {
        // This would fail to compile because name is missing:
        // let person = PersonBuilder::new().age(25).build();

        let person = PersonBuilder::new()
            .name("Alice".to_string())
            .age(25)
            .build();

        assert_eq!(person.name, "Alice");
        assert_eq!(person.age, 25);
    }
}

// Key Benefits of PhantomData approach:
//
// 1. **Zero Runtime Cost**: PhantomData<T> is zero-sized, so it has no runtime overhead
// 2. **Compile-time Safety**: Invalid states/operations are caught at compile time
// 3. **No Dynamic Dispatch**: Unlike Box<dyn Trait>, everything is resolved at compile time
// 4. **Type-level Programming**: You can encode logic in the type system itself
// 5. **Memory Efficiency**: No heap allocations needed
// 6. **Performance**: No virtual function calls or dynamic dispatch
//
// When to use PhantomData instead of Box<dyn>:
// - When you have a finite set of types/behaviors known at compile time
// - When you want to enforce state transitions at compile time
// - When you need type-level guarantees about API usage
// - When performance is critical and you want zero-cost abstractions
// - When you want to prevent certain operations based on type state
//
// When to still use Box<dyn>:
// - When dealing with truly unknown types at compile time
// - When you need runtime polymorphism
// - When working with plugin systems or dynamic loading
// - When the set of implementations is not known at compile time
