package fr.insa.cours.devops.coffee.machine.component;

import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

public class CoffeeGrinder {
    public static final Logger logger = LogManager.getLogger(CoffeeGrinder.class);
    private final int grindingTime;

    /**
     * Constructeur du moulin à café intégré dans la machine
     * @param grindingTime Temps que met le moulin pour moudre le café en millisecondes
     */
    public CoffeeGrinder(int grindingTime) {
        this.grindingTime = grindingTime;
    }

    /**
     * Moue le volume de café spécifié dans le réservoir beanTank
     * @param beanTank Réservoir de café
     * @return Temps pour moudre le café
     * @throws InterruptedException Exception levée en cas de problèmes lors du sleep par le Thread
     */
    public double grindCoffee(BeanTank beanTank) throws InterruptedException {
        if (beanTank.getActualVolume() < 0.2) {
            throw new IllegalArgumentException("Not enough coffee beans in the tank to grind.");
        }
        logger.info("Grinding time : "  +  grindingTime);
        logger.info("Grinding...");
        Thread.sleep(grindingTime);
        beanTank.decreaseVolumeInTank(0.2);
        logger.info("Grinding OK");
        return grindingTime;
    }
}
