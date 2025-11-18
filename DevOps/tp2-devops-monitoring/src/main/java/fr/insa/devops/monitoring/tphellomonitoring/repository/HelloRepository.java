
package fr.insa.devops.monitoring.tphellomonitoring.repository;

import fr.insa.devops.monitoring.tphellomonitoring.model.HelloObject;
import org.springframework.data.jpa.repository.JpaRepository;

public interface HelloRepository extends JpaRepository<HelloObject, Long> {
}
