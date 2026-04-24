use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod guarderia {
    use super::*;

    pub fn crear(ctx: Context<Crear>, nombre: String, direccion: String) -> Result<()> {
        let data = &mut ctx.accounts.guarderia;
        data.nombre = nombre;
        data.direccion = direccion;
        Ok(())
    }

    pub fn actualizar(ctx: Context<Actualizar>, nombre: String, direccion: String) -> Result<()> {
        let data = &mut ctx.accounts.guarderia;
        data.nombre = nombre;
        data.direccion = direccion;
        Ok(())
    }

    pub fn eliminar(_ctx: Context<Eliminar>) -> Result<()> {
        Ok(())
    }
}

#[account]
pub struct Guarderia {
    pub nombre: String,
    pub direccion: String,
}

#[derive(Accounts)]
pub struct Crear<'info> {
    #[account(init, payer = user, space = 8 + 100)]
    pub guarderia: Account<'info, Guarderia>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Actualizar<'info> {
    #[account(mut)]
    pub guarderia: Account<'info, Guarderia>,
}

#[derive(Accounts)]
pub struct Eliminar<'info> {
    #[account(mut, close = user)]
    pub guarderia: Account<'info, Guarderia>,

    #[account(mut)]
    pub user: Signer<'info>,
}
