package fr.insa.devops.monitoring.tphellomonitoring.service;

import fr.insa.devops.monitoring.tphellomonitoring.model.HelloObject;
import fr.insa.devops.monitoring.tphellomonitoring.repository.HelloRepository;
import io.micrometer.core.annotation.Counted;
import jakarta.persistence.EntityNotFoundException;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

@Service
public class HelloService {
    @Autowired
    private HelloRepository helloRepository;

    @Counted(value = "helloservice.hello.gethellobyid", description = "Number of times getHelloById is called")
    public HelloObject getHelloById(Long id) {
        return this.helloRepository.findById(id).orElseThrow(
                ()-> new EntityNotFoundException("Hello not found with id " + id));
    }

    public HelloObject saveHello(HelloObject helloObject) {
        return this.helloRepository.save(helloObject);
    }
}
