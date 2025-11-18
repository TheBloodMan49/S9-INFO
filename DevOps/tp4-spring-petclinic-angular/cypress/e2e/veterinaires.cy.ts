describe('template spec', () => {
  it('passes', () => {
    cy.visit('http://localhost:4200')
    cy.get('a.dropdown-toggle').contains('Veterinarians').click()
    cy.get('a').contains('All').click()
    cy.get('tbody tr').should('have.length',6)
  })
})