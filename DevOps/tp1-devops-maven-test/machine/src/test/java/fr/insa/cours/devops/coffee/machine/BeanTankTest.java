package fr.insa.cours.devops.coffee.machine;

import fr.insa.cours.devops.coffee.cupboard.type.CoffeeType;
import fr.insa.cours.devops.coffee.machine.component.BeanTank;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class BeanTankTest {

    @Test
    public void testBeanTankCreation(){
        BeanTank beanTank = new BeanTank(1.0, 0.2, 3.0, CoffeeType.ARABICA);
        Assertions.assertEquals(CoffeeType.ARABICA, beanTank.getBeanCoffeeType());
    }

    @Test
    public void testIncreaseCoffeeVolumeInTank(){
        BeanTank beanTank = new BeanTank(1.0, 0.2, 3.0, CoffeeType.ARABICA);
        beanTank.increaseCoffeeVolumeInTank(1.0, CoffeeType.ROBUSTA);
        Assertions.assertEquals(2.0, beanTank.getActualVolume());
        Assertions.assertEquals(CoffeeType.ROBUSTA, beanTank.getBeanCoffeeType());
    }
}
