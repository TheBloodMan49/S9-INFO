package fr.insa.cours.devops.coffee.machine.cucumber.steps;

import io.cucumber.java.en.Given;
import io.cucumber.java.en.Then;
import io.cucumber.java.en.When;

import static org.junit.Assert.assertEquals;

public class StepsAdditionTest {

    private double number1;
    private double number2;
    private double result;

    @Given("two numbers {double} and {double}")
    public void two_numbers_and(double num1, double num2) {
        this.number1 = num1;
        this.number2 = num2;
    }

    @When("I add them")
    public void i_add_them() {
        this.result = this.number1 + this.number2;
    }

    @Then("the result should be {double}")
    public void the_result_should_be(double expectedResult) {
        assertEquals(expectedResult, this.result, 0.0001);
    }


}
