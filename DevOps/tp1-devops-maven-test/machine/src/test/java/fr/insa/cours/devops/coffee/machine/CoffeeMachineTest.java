package fr.insa.cours.devops.coffee.machine;

import fr.insa.cours.devops.coffee.cupboard.FabricCupboardContainer;
import fr.insa.cours.devops.coffee.cupboard.container.Container;
import fr.insa.cours.devops.coffee.cupboard.type.CoffeeType;
import fr.insa.cours.devops.coffee.cupboard.container.Mug;
import fr.insa.cours.devops.coffee.cupboard.exception.CupNotEmptyException;
import fr.insa.cours.devops.coffee.cupboard.exception.ExceptionContainerCreation;
import fr.insa.cours.devops.coffee.machine.component.WaterTank;
import fr.insa.cours.devops.coffee.machine.exception.CannotMakeCremaWithSimpleCoffeeMachine;
import fr.insa.cours.devops.coffee.machine.exception.CoffeeTypeCupDifferentOfCoffeeTypeTankException;
import fr.insa.cours.devops.coffee.machine.exception.LackOfWaterInTankException;
import fr.insa.cours.devops.coffee.machine.exception.MachineNotPluggedException;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Assertions;
import org.mockito.Mockito;

public class CoffeeMachineTest {
    CoffeeMachine spyCoffeeMachine;
    Mug mug;

    private double minCoffeeBeanTankVolume = 0.2;
    private double maxCoffeeBeanTankVolume = 3.0;
    private double minWaterTankVolume = 0.2;
    private double maxWaterTankVolume = 3.0;
    private double pumpingCapacity = 5000;

    @BeforeEach
    public void setUp() throws ExceptionContainerCreation {
        CoffeeMachine coffeeMachine = new CoffeeMachine(minCoffeeBeanTankVolume, maxCoffeeBeanTankVolume, minWaterTankVolume, maxWaterTankVolume, pumpingCapacity);
        mug = (Mug) FabricCupboardContainer.getFabricContainerInstance().getContainer("mug", 0.3);

        spyCoffeeMachine = Mockito.spy(coffeeMachine);
        Mockito.doNothing().when(spyCoffeeMachine).coffeeMachineFailure();
    }

    @Test
    public void testCoffeeMachineCreation() {
        Assertions.assertNotNull(spyCoffeeMachine);
        Assertions.assertNotNull(spyCoffeeMachine.getBeanTank());
        Assertions.assertNotNull(spyCoffeeMachine.getWaterTank());
        Assertions.assertNotNull(spyCoffeeMachine.getCoffeeGrinder());
        Assertions.assertEquals(pumpingCapacity/3600, spyCoffeeMachine.getWaterPump().getPumpingCapacity(), 0.01);
    }

    @Test
    public void testFillWaterTank() {
        WaterTank waterTank = spyCoffeeMachine.getWaterTank();
        double initialVolume = waterTank.getActualVolume();
        double fillVolume = 1.0;
        spyCoffeeMachine.addWaterInTank(fillVolume);
        Assertions.assertEquals(Math.min(initialVolume + fillVolume, maxWaterTankVolume), waterTank.getActualVolume());
    }

    @Test
    public void testMakeCoffee() throws InterruptedException, CoffeeTypeCupDifferentOfCoffeeTypeTankException, LackOfWaterInTankException, CupNotEmptyException, CannotMakeCremaWithSimpleCoffeeMachine, MachineNotPluggedException {
        double initialWaterVolume = spyCoffeeMachine.getWaterTank().getActualVolume();
        double initialBeanVolume = spyCoffeeMachine.getBeanTank().getActualVolume();
        double coffeeVolume = 0.3;
        double beanVolume = 0.2;

        spyCoffeeMachine.addWaterInTank(coffeeVolume+1.0);
        spyCoffeeMachine.addCoffeeInBeanTank(beanVolume+1.0, CoffeeType.ARABICA);
        spyCoffeeMachine.plugToElectricalPlug();

        Container newmug = spyCoffeeMachine.makeACoffee(mug, CoffeeType.ARABICA);

        Assertions.assertEquals(initialWaterVolume+1.0, spyCoffeeMachine.getWaterTank().getActualVolume(), 0.01);
        Assertions.assertEquals(initialBeanVolume+1.0, spyCoffeeMachine.getBeanTank().getActualVolume(), 0.01);

        Assertions.assertEquals(coffeeVolume, newmug.getCapacity(), 0.01);
    }
}
