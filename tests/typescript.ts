/*
 * JAVA/JS/C/TS style comments example
 *
 * Note that this comment has to be single * because it is treated as inner comment
 */

/// Single line comment
function hello() {
    console.log('Hello World');
}

/**
 * Java
 */
public class HelloWorld {
    /*
     * Inner comment
     */

    /**
     * Main method
     * @param args Command line arguments
     * @return void
     */
    public static void main(String[] args) {
        System.out.println("Hello World");
    }
}

/**
 * ES6 class
 */
export class Hello {
    /**
     * Constructor
     * @param name Name
     */
    constructor(name: string) {
        this.name = name;

        // double-slash is not doc comment so you can't find this one
        hello();

        /// You can find anonymous function/classes too if 
        /// they are documented properly, like this one
        function hello() {
            console.log('Hello ' + this.name);

            /** the nesting can go on forever */
            for (let i = 0; i < 10; i++) {
                console.log('Hello ' + this.name);
            }
        }

        /// (need to manually end the section here, otherwise the stuff below will
        /// be considered part of the anonymous function by the tool)
        /// also need the statement below so that this doc comment is recognized
        console.log('Hello ' + this.name);
    }
}
