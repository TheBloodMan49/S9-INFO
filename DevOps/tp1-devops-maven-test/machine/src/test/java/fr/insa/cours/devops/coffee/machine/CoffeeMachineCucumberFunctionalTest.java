package fr.insa.cours.devops.coffee.machine;

import io.cucumber.junit.Cucumber;
import io.cucumber.junit.CucumberOptions;
import org.junit.Ignore;
import org.junit.runner.RunWith;

@RunWith(Cucumber.class)
@CucumberOptions(
        features = {"classpath:/features/"},
        glue = "fr.insa.cours.devops.coffee.machine.cucumber.steps"
)
//@Ignore
public class CoffeeMachineCucumberFunctionalTest {

}
