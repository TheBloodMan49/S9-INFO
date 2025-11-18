package fr.insa.cours.devops.coffee.machine;

import fr.insa.cours.devops.coffee.cupboard.type.CoffeeType;
import fr.insa.cours.devops.coffee.machine.component.BeanTank;
import fr.insa.cours.devops.coffee.machine.component.CoffeeGrinder;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.BeforeEach;

public class CoffeeGrinderTest {
    CoffeeGrinder coffeeGrinder;
    BeanTank beanTank;

    static int grindingTime = 5;

    @BeforeEach
    public void setup() {
        coffeeGrinder = new CoffeeGrinder(grindingTime);
        beanTank = new BeanTank(0, 0.2, 3.0, null);
    }

    @Test
    public void testGrindCoffee() throws InterruptedException {
        double coffeeVolume = 2.0;
        beanTank.increaseCoffeeVolumeInTank(coffeeVolume, CoffeeType.ROBUSTA);
        double actualGrindingTime = coffeeGrinder.grindCoffee(beanTank);
        Assertions.assertEquals(grindingTime, actualGrindingTime);
        Assertions.assertEquals(coffeeVolume - 0.2, beanTank.getActualVolume());
    }
}
