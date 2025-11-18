package fr.insa.cours.devops.coffee.machine;

import fr.insa.cours.devops.coffee.machine.component.Tank;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public class TankTest {

    Tank tank;

    @BeforeEach
    public void setup() {
        tank = new Tank(1.0, 0.2, 3.0);
    }

    @Test
    public void testTankCreation(){
        Assertions.assertEquals(1.0, tank.getActualVolume());
        Assertions.assertEquals(0.2, tank.getMinVolume());
        Assertions.assertEquals(3.0, tank.getMaxVolume());
    }

    @Test
    public void testIncreaseVolumeInTank(){
        tank.increaseVolumeInTank(1.0);
        Assertions.assertEquals(2.0, tank.getActualVolume());
    }

    @Test
    public void testDecreaseVolumeInTank(){
        tank.decreaseVolumeInTank(0.5);
        Assertions.assertEquals(0.5, tank.getActualVolume());
    }

    @Test
    public void testVolumeMaxLimit(){
        tank.increaseVolumeInTank(3.0);
        Assertions.assertEquals(3.0, tank.getActualVolume());
    }

    @Test
    public void testVolumeMinLimit(){
        tank.decreaseVolumeInTank(2.0);
        Assertions.assertEquals(0.2, tank.getActualVolume(), 0.01);
    }
}
