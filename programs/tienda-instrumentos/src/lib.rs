use anchor_lang::prelude::*;

declare_id!("4uzyBjtkwHAHTZyHA6aHJERyGALTs2fJrZsvJHAN5fmM");

#[program]
pub mod tienda_instrumentos {
    use super::*;

    pub fn crear_tienda(ctx: Context<CrearTienda>, nombre: String) -> Result<()> {
        let tienda = &mut ctx.accounts.tienda;
        tienda.nombre = nombre;
        tienda.owner = *ctx.accounts.owner.key;
        tienda.instrumentos = Vec::new();
        Ok(())
    }

    pub fn agregar_instrumento(
        ctx: Context<AgregarInstrumento>,
        marca: String,
        modelo: String,
        precio: u64,
        en_stock: bool,
    ) -> Result<()> {
        let tienda = &mut ctx.accounts.tienda;

        let nuevo_instrumento = Instrumento {
            marca,
            modelo,
            precio,
            en_stock,
        };

        tienda.instrumentos.push(nuevo_instrumento);
        Ok(())
    }

    pub fn eliminar_instrumento(
        ctx: Context<EliminarInstrumento>,
        modelo: String,
    ) -> Result<()> {
        let tienda = &mut ctx.accounts.tienda;

        if let Some(pos) = tienda
            .instrumentos
            .iter()
            .position(|i| i.modelo == modelo)
        {
            tienda.instrumentos.swap_remove(pos);
            Ok(())
        } else {
            Err(ErrorCode::InstrumentoNoEncontrado.into())
        }
    }

    pub fn cambiar_disponibilidad(ctx: Context<CambiarDisponibilidad>, modelo: String) -> Result<()> {
        let tienda = &mut ctx.accounts.tienda;

        if let Some(instrumento) = tienda
            .instrumentos
            .iter_mut()
            .find(|i| i.modelo == modelo)
        {
            instrumento.en_stock = !instrumento.en_stock;
            Ok(())
        } else {
            Err(ErrorCode::InstrumentoNoEncontrado.into())
        }
    }

    // Nueva función para ver los instrumentos
    pub fn ver_instrumentos(ctx: Context<VerInstrumentos>) -> Result<()> {
        let tienda = &ctx.accounts.tienda;

        if tienda.instrumentos.is_empty() {
            msg!("La tienda está vacía.");
            return Ok(());
        }

        msg!("Listado de instrumentos en la tienda '{}':", tienda.nombre);

        for (i, inst) in tienda.instrumentos.iter().enumerate() {
            msg!(
                "#{}: {} {} | Precio: {} | Stock: {}",
                i + 1,
                inst.marca,
                inst.modelo,
                inst.precio,
                inst.en_stock
            );
        }

        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Instrumento no encontrado en el inventario.")]
    InstrumentoNoEncontrado,
}

#[derive(Accounts)]
pub struct CrearTienda<'info> {
    #[account(
        init, 
        payer = owner, 
        space = 8 + TiendaInstrumentos::INIT_SPACE,
        seeds = [b"tienda", owner.key().as_ref()],
        bump
    )]
    pub tienda: Account<'info, TiendaInstrumentos>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct TiendaInstrumentos {
    #[max_len(100)]
    pub nombre: String,
    pub owner: Pubkey,
    #[max_len(20)]
    pub instrumentos: Vec<Instrumento>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Instrumento {
    #[max_len(60)]
    pub marca: String,
    #[max_len(60)]
    pub modelo: String,
    pub precio: u64,
    pub en_stock: bool,
}

#[derive(Accounts)]
pub struct AgregarInstrumento<'info> {
    #[account(mut, has_one = owner)]
    pub tienda: Account<'info, TiendaInstrumentos>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct EliminarInstrumento<'info> {
    #[account(mut, has_one = owner)]
    pub tienda: Account<'info, TiendaInstrumentos>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct CambiarDisponibilidad<'info> {
    #[account(mut, has_one = owner)]
    pub tienda: Account<'info, TiendaInstrumentos>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerInstrumentos<'info> {
    pub tienda: Account<'info, TiendaInstrumentos>,
}