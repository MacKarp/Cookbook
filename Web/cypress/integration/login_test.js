describe('Login page test', () => {
  it('Visiting cookbook', () => {
    cy.visit('/')
    cy.contains('Log in').click()
    cy.url().should('include', '/login')
    
  })
  it('Log in and out', () => {
    cy.visit('/login')
    cy.get('input[type=email]').type('hermano@gmail.com')
      .should('have.value', 'hermano@gmail.com')

    cy.get('input[type=password]').type('hermano1234')
      .should('have.value', 'hermano1234')
      
    cy.get('.loginButton').click()
    
    cy.pause()
      
    cy.contains('Logout').click()
    cy.url().should('include', '/login')
  
  })
})
