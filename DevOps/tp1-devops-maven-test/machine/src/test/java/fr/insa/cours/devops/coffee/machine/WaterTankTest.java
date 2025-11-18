package fr.insa.cours.devops.coffee.machine;

import fr.insa.cours.devops.coffee.machine.component.WaterTank;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Assertions;

public class WaterTankTest {
    WaterTank waterTank;

    @BeforeEach
    public void setUp() {
        waterTank = new WaterTank(1.0, 0.2, 3.0);
    }

    @Test
    public void testConstruction() {
        Assertions.assertEquals(1.0, waterTank.getActualVolume());
        Assertions.assertEquals(0.2, waterTank.getMinVolume());
        Assertions.assertEquals(3.0, waterTank.getMaxVolume());
    }

    @Test
    public void testIncreaseVolume() {
        waterTank.increaseVolumeInTank(1.0);
        Assertions.assertEquals(2.0, waterTank.getActualVolume());
    }

    @Test
    public void testDecreaseVolume() {
        waterTank.decreaseVolumeInTank(0.5);
        Assertions.assertEquals(0.5, waterTank.getActualVolume());
    }
}
