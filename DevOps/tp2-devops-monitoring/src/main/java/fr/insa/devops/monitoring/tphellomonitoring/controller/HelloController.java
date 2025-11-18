package fr.insa.devops.monitoring.tphellomonitoring.controller;

import fr.insa.devops.monitoring.tphellomonitoring.dto.HelloDTO;
import fr.insa.devops.monitoring.tphellomonitoring.model.HelloObject;
import fr.insa.devops.monitoring.tphellomonitoring.service.HelloService;
import io.micrometer.core.annotation.Counted;
import org.springframework.web.bind.annotation.*;

@RestController
public class HelloController {

    private final HelloService helloService;

    public HelloController(HelloService helloService) {
        this.helloService = helloService;
    }

    @Counted(value = "hellocontroller.hello.get", description = "Number of times the /hello GET without ID is called")
    @GetMapping("/hello")
    public String hello(String param){
        return "Hello " + param;
    }

    @PostMapping("/hello")
    public HelloDTO create(@RequestBody HelloDTO helloDto){
        HelloObject hello = helloService.saveHello(new HelloObject(helloDto.param()));
        return new HelloDTO(hello.getId(), hello.getParam());
    }

    @GetMapping("/hello/{id}")
    public HelloDTO get(@PathVariable Long id){
        HelloObject hello = helloService.getHelloById(id);
        return new HelloDTO(hello.getId(), hello.getParam());
    }
}
