package fr.insa.cours.devops.coffee.machine;

import fr.insa.cours.devops.coffee.machine.component.WaterPump;
import fr.insa.cours.devops.coffee.machine.component.WaterTank;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public class WaterPumpTest {

    WaterPump waterPump;
    WaterTank waterTank;

    final double PUMPING_CAP = 3.0;

    private double estimatedPumpingTime(double volume) {
        return (volume / PUMPING_CAP) * 1000 * 2;
    }

    @BeforeEach
    public void setup() {
        waterPump = new WaterPump(PUMPING_CAP);
        waterTank = new WaterTank(1.0, 0.2, 5.0);
    }

    @Test
    public void testConstructor() {
        Assertions.assertEquals(PUMPING_CAP, waterPump.getPumpingCapacity());
    }

    @Test
    public void testPumpWater() throws InterruptedException {
        double initialVolume = waterTank.getActualVolume();
        double volumeToPump = 0.5;
        double expectedTime = estimatedPumpingTime(volumeToPump);

        double actualTime = waterPump.pumpWater(volumeToPump, waterTank);

        Assertions.assertEquals(expectedTime, actualTime);
        Assertions.assertEquals(initialVolume - volumeToPump, waterTank.getActualVolume(), 0.01);
    }

    @Test
    public void testPumpTooMuch() throws InterruptedException {
        double initialVolume = waterTank.getActualVolume();
        double volumeToPump = 10.0; // More than available

        Assertions.assertThrows(IllegalArgumentException.class, () -> {
            waterPump.pumpWater(volumeToPump, waterTank);
        });
    }

    @Test
    public void testPumpNegativeVolume() {
        double volumeToPump = -1.0; // Negative volume

        Assertions.assertThrows(IllegalArgumentException.class, () -> {
            waterPump.pumpWater(volumeToPump, waterTank);
        });
    }
}
