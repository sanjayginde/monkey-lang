// Comparison: Box<dyn> vs PhantomData approaches
// This file demonstrates the key differences between runtime and compile-time polymorphism

use std::marker::PhantomData;

// ===== APPROACH 1: Box<dyn> - Runtime Polymorphism =====

trait RuntimeProcessor {
    fn process(&self, input: &str) -> String;
    fn name(&self) -> &'static str;
}

struct UpperCaseProcessor;
struct LowerCaseProcessor;
struct ReverseProcessor;

impl RuntimeProcessor for UpperCaseProcessor {
    fn process(&self, input: &str) -> String {
        input.to_uppercase()
    }

    fn name(&self) -> &'static str {
        "UpperCase"
    }
}

impl RuntimeProcessor for LowerCaseProcessor {
    fn process(&self, input: &str) -> String {
        input.to_lowercase()
    }

    fn name(&self) -> &'static str {
        "LowerCase"
    }
}

impl RuntimeProcessor for ReverseProcessor {
    fn process(&self, input: &str) -> String {
        input.chars().rev().collect()
    }

    fn name(&self) -> &'static str {
        "Reverse"
    }
}

// Runtime polymorphic container - uses Box<dyn>
struct RuntimeProcessorChain {
    processors: Vec<Box<dyn RuntimeProcessor>>,
}

impl RuntimeProcessorChain {
    fn new() -> Self {
        Self {
            processors: Vec::new(),
        }
    }

    fn add_processor(&mut self, processor: Box<dyn RuntimeProcessor>) {
        self.processors.push(processor);
    }

    fn process(&self, input: &str) -> String {
        self.processors
            .iter()
            .fold(input.to_string(), |acc, processor| processor.process(&acc))
    }

    fn get_processor_names(&self) -> Vec<&'static str> {
        self.processors.iter().map(|p| p.name()).collect()
    }
}

// ===== APPROACH 2: PhantomData - Compile-Time Polymorphism =====

// Marker types for different processing strategies
#[derive(Debug)]
struct UpperCase;

#[derive(Debug)]
struct LowerCase;

#[derive(Debug)]
struct Reverse;

// Trait that works at compile time
trait CompileTimeProcessor {
    fn process(input: &str) -> String;
    fn name() -> &'static str;
}

impl CompileTimeProcessor for UpperCase {
    fn process(input: &str) -> String {
        input.to_uppercase()
    }

    fn name() -> &'static str {
        "UpperCase"
    }
}

impl CompileTimeProcessor for LowerCase {
    fn process(input: &str) -> String {
        input.to_lowercase()
    }

    fn name() -> &'static str {
        "LowerCase"
    }
}

impl CompileTimeProcessor for Reverse {
    fn process(input: &str) -> String {
        input.chars().rev().collect()
    }

    fn name() -> &'static str {
        "Reverse"
    }
}

// Compile-time polymorphic processor using PhantomData
struct CompileTimeProcessorWrapper<T: CompileTimeProcessor> {
    _marker: PhantomData<T>,
}

impl<T: CompileTimeProcessor> CompileTimeProcessorWrapper<T> {
    fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    fn process(&self, input: &str) -> String {
        T::process(input)
    }

    fn name(&self) -> &'static str {
        T::name()
    }
}

// Type aliases for convenience
type UpperCaseProcessor2 = CompileTimeProcessorWrapper<UpperCase>;
type LowerCaseProcessor2 = CompileTimeProcessorWrapper<LowerCase>;
type ReverseProcessor2 = CompileTimeProcessorWrapper<Reverse>;

// ===== APPROACH 3: PhantomData State Machine =====

// State types
#[derive(Debug)]
struct Unprocessed;

#[derive(Debug)]
struct Processed<T> {
    _processor_type: PhantomData<T>,
}

// Data container that tracks processing state at compile time
struct ProcessingPipeline<State> {
    data: String,
    _state: PhantomData<State>,
}

impl ProcessingPipeline<Unprocessed> {
    fn new(data: String) -> Self {
        Self {
            data,
            _state: PhantomData,
        }
    }

    // Each processing step changes the type
    fn apply_uppercase(self) -> ProcessingPipeline<Processed<UpperCase>> {
        ProcessingPipeline {
            data: self.data.to_uppercase(),
            _state: PhantomData,
        }
    }

    fn apply_lowercase(self) -> ProcessingPipeline<Processed<LowerCase>> {
        ProcessingPipeline {
            data: self.data.to_lowercase(),
            _state: PhantomData,
        }
    }

    fn apply_reverse(self) -> ProcessingPipeline<Processed<Reverse>> {
        ProcessingPipeline {
            data: self.data.chars().rev().collect(),
            _state: PhantomData,
        }
    }
}

impl<T> ProcessingPipeline<Processed<T>> {
    // Only processed data can be finalized
    fn finalize(self) -> String {
        self.data
    }

    // Chain additional processing
    fn then_uppercase(self) -> ProcessingPipeline<Processed<UpperCase>> {
        ProcessingPipeline {
            data: self.data.to_uppercase(),
            _state: PhantomData,
        }
    }

    fn then_lowercase(self) -> ProcessingPipeline<Processed<LowerCase>> {
        ProcessingPipeline {
            data: self.data.to_lowercase(),
            _state: PhantomData,
        }
    }

    fn then_reverse(self) -> ProcessingPipeline<Processed<Reverse>> {
        ProcessingPipeline {
            data: self.data.chars().rev().collect(),
            _state: PhantomData,
        }
    }
}

// ===== PERFORMANCE AND USAGE COMPARISON =====

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_runtime_polymorphism() {
        let mut chain = RuntimeProcessorChain::new();
        chain.add_processor(Box::new(UpperCaseProcessor));
        chain.add_processor(Box::new(ReverseProcessor));
        chain.add_processor(Box::new(LowerCaseProcessor));

        let result = chain.process("Hello World");
        println!("Runtime result: {}", result);

        let names = chain.get_processor_names();
        println!("Runtime processors: {:?}", names);

        // Note: This approach allows dynamic composition at runtime
        // but has overhead of heap allocation and dynamic dispatch
    }

    #[test]
    fn test_compile_time_polymorphism() {
        let upper_processor = UpperCaseProcessor2::new();
        let reverse_processor = ReverseProcessor2::new();
        let lower_processor = LowerCaseProcessor2::new();

        let input = "Hello World";
        let step1 = upper_processor.process(input);
        let step2 = reverse_processor.process(&step1);
        let result = lower_processor.process(&step2);

        println!("Compile-time result: {}", result);
        println!(
            "Processors used: {}, {}, {}",
            upper_processor.name(),
            reverse_processor.name(),
            lower_processor.name()
        );

        // Note: This approach has zero runtime overhead
        // but the processing chain must be known at compile time
    }

    #[test]
    fn test_phantom_data_state_machine() {
        let pipeline = ProcessingPipeline::new("Hello World".to_string());

        // This would fail to compile: pipeline.finalize();
        // because unprocessed data cannot be finalized

        let result = pipeline
            .apply_uppercase()
            .then_reverse()
            .then_lowercase()
            .finalize();

        println!("State machine result: {}", result);

        // Note: This approach prevents invalid state transitions at compile time
        // and has zero runtime overhead
    }

    #[test]
    fn performance_comparison() {
        let iterations = 100_000;
        let input = "Hello World! This is a test string for performance comparison.";

        // Runtime polymorphism benchmark
        let start = Instant::now();
        for _ in 0..iterations {
            let mut chain = RuntimeProcessorChain::new();
            chain.add_processor(Box::new(UpperCaseProcessor));
            chain.add_processor(Box::new(ReverseProcessor));
            let _result = chain.process(input);
        }
        let runtime_duration = start.elapsed();

        // Compile-time polymorphism benchmark
        let start = Instant::now();
        for _ in 0..iterations {
            let upper_processor = UpperCaseProcessor2::new();
            let reverse_processor = ReverseProcessor2::new();
            let step1 = upper_processor.process(input);
            let _result = reverse_processor.process(&step1);
        }
        let compile_time_duration = start.elapsed();

        println!("Runtime polymorphism: {:?}", runtime_duration);
        println!("Compile-time polymorphism: {:?}", compile_time_duration);
        println!(
            "Speedup: {:.2}x",
            runtime_duration.as_nanos() as f64 / compile_time_duration.as_nanos() as f64
        );
    }
}

// ===== KEY DIFFERENCES SUMMARY =====

/*
Box<dyn Trait> Approach:
✅ Pros:
  - Dynamic composition at runtime
  - Can store different types in same collection
  - Flexible for plugin systems
  - Unknown types until runtime

❌ Cons:
  - Heap allocation overhead
  - Dynamic dispatch (virtual function calls)
  - Runtime performance cost
  - Larger memory footprint

PhantomData<T> Approach:
✅ Pros:
  - Zero runtime overhead
  - Static dispatch (inlined by compiler)
  - Compile-time safety guarantees
  - Type-level state tracking
  - Memory efficient (no heap allocations)

❌ Cons:
  - Types must be known at compile time
  - Less flexible for dynamic scenarios
  - Cannot store different types in same collection
  - More complex type signatures

When to use Box<dyn>:
- Plugin systems
- Configuration-driven behavior
- Truly dynamic polymorphism needed
- Types not known until runtime

When to use PhantomData:
- Performance-critical code
- State machines with compile-time validation
- Type-safe APIs
- Zero-cost abstractions needed
- Finite set of known types
*/

// ===== ADVANCED PHANTOM DATA PATTERNS =====

// Pattern 1: Type-level computation
struct Multiply<A, B> {
    _a: PhantomData<A>,
    _b: PhantomData<B>,
}

// Pattern 2: Branded types (preventing mixing of different ID types)
struct UserId(u64);
struct PostId(u64);

struct TypedId<T> {
    id: u64,
    _type: PhantomData<T>,
}

type UserIdTyped = TypedId<UserId>;
type PostIdTyped = TypedId<PostId>;

// This prevents accidentally mixing user IDs and post IDs
fn get_user_by_id(_id: UserIdTyped) -> String {
    "User".to_string()
}

// This would fail to compile:
// get_user_by_id(PostIdTyped { id: 1, _type: PhantomData });

// Pattern 3: Linear types (use-once semantics)
struct LinearResource<State> {
    data: String,
    _state: PhantomData<State>,
}

struct Acquired;
struct Released;

impl LinearResource<Acquired> {
    fn use_resource(self) -> (String, LinearResource<Released>) {
        let data = self.data.clone();
        (
            data,
            LinearResource {
                data: self.data,
                _state: PhantomData,
            },
        )
    }
}

impl LinearResource<Released> {
    fn cleanup(self) {
        // Resource cleanup logic
        println!("Cleaning up resource: {}", self.data);
    }
}
