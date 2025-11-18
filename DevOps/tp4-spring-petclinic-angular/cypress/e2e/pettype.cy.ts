describe('template spec', () => {
  it('passes', () => {
    cy.visit('http://localhost:4200')
    cy.get('a').contains(/Pet Types/i).click()
  })
})