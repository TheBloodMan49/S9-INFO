package fr.insa.cours.devops.coffee.machine.exception;

public class LackOfWaterInTankException extends Exception{
    public LackOfWaterInTankException(String message) {
        super(message);
    }
}
