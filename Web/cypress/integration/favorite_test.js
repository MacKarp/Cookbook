describe('Favorite', () => {
    it('Visiting cookbook', () => {
        cy.clearCookies()
        const password = 'cypress1234'
        const loginName = 'testCypress@cy.pl'
        cy.visit('/login')
        
        cy.get('input[type=email]').type(loginName)
        cy.get('input[type=password]').type(`${password}{enter}`)
        cy.contains('Buongiorno!')
    
    })
    it('Go to Search by name', () => {
        cy.contains('Search by name').click()
        cy.url().should('include', '/byName')
    })
    it('Search for the meal (Pierogi)', () => {
        cy.get('input').type(`pierogi{enter}`)
        cy.get('.linkArea').click()
    })    
    it('Add meal to favorites', () => {
        cy.contains('Add to your favorites').click()
        
    })
    it('Go to favorites', () => {
        cy.get('.play').click()
        cy.url().should('include', '/favorites')
        cy.wait(3000)
        cy.get('.favoritesList').should('contain', 'Pierogi')
        
            .as('pierogi')
        
        cy.get('@pierogi').click()
        
    })
    it('Go to meal details (pierogi) from favorites', () => {
        cy.url().should('include', '/meals/53019')
    })
  })