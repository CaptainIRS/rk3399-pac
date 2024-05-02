#[doc = "Register `MIPI_ISR` writer"]
pub type W = crate::W<MipiIsrSpec>;
#[doc = "Field `ISR_ERR_SOT` writer - 1: set register; 0: nothing happens (one bit for each\n\nlane)"]
pub type IsrErrSotW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ISR_ERR_SOT_SYNC` writer - 1: set register; 0: nothing happens (one bit for each\n\nlane)"]
pub type IsrErrSotSyncW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ISR_ERR_EOT_SYNC` writer - 1: set register; 0: nothing happens (one bit for each\n\nlane)"]
pub type IsrErrEotSyncW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ISR_ERR_CONTROL` writer - 1: set register; 0: nothing happens (one bit for each\n\nlane)"]
pub type IsrErrControlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ISR_ERR_PROTOCOL` writer - 1: set register; 0: nothing happens"]
pub type IsrErrProtocolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_ERR_ECC2` writer - 1: set register; 0: nothing happens"]
pub type IsrErrEcc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_ERR_ECC1` writer - 1: set register; 0: nothing happens"]
pub type IsrErrEcc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_ERR_CS` writer - 1: set register; 0: nothing happens"]
pub type IsrErrCsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_FRAME_END` writer - 1: set register; 0: nothing happens"]
pub type IsrFrameEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_ADD_DATA_OVFLW` writer - 1: set register; 0: nothing happens"]
pub type IsrAddDataOvflwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_GEN_SHORT_PACK` writer - 1: set register; 0: nothing happens"]
pub type IsrGenShortPackW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 4:7 - 1: set register; 0: nothing happens (one bit for each\n\nlane)"]
    #[inline(always)]
    #[must_use]
    pub fn isr_err_sot(&mut self) -> IsrErrSotW<MipiIsrSpec> {
        IsrErrSotW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 1: set register; 0: nothing happens (one bit for each\n\nlane)"]
    #[inline(always)]
    #[must_use]
    pub fn isr_err_sot_sync(&mut self) -> IsrErrSotSyncW<MipiIsrSpec> {
        IsrErrSotSyncW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 1: set register; 0: nothing happens (one bit for each\n\nlane)"]
    #[inline(always)]
    #[must_use]
    pub fn isr_err_eot_sync(&mut self) -> IsrErrEotSyncW<MipiIsrSpec> {
        IsrErrEotSyncW::new(self, 12)
    }
    #[doc = "Bits 16:19 - 1: set register; 0: nothing happens (one bit for each\n\nlane)"]
    #[inline(always)]
    #[must_use]
    pub fn isr_err_control(&mut self) -> IsrErrControlW<MipiIsrSpec> {
        IsrErrControlW::new(self, 16)
    }
    #[doc = "Bit 20 - 1: set register; 0: nothing happens"]
    #[inline(always)]
    #[must_use]
    pub fn isr_err_protocol(&mut self) -> IsrErrProtocolW<MipiIsrSpec> {
        IsrErrProtocolW::new(self, 20)
    }
    #[doc = "Bit 21 - 1: set register; 0: nothing happens"]
    #[inline(always)]
    #[must_use]
    pub fn isr_err_ecc2(&mut self) -> IsrErrEcc2W<MipiIsrSpec> {
        IsrErrEcc2W::new(self, 21)
    }
    #[doc = "Bit 22 - 1: set register; 0: nothing happens"]
    #[inline(always)]
    #[must_use]
    pub fn isr_err_ecc1(&mut self) -> IsrErrEcc1W<MipiIsrSpec> {
        IsrErrEcc1W::new(self, 22)
    }
    #[doc = "Bit 23 - 1: set register; 0: nothing happens"]
    #[inline(always)]
    #[must_use]
    pub fn isr_err_cs(&mut self) -> IsrErrCsW<MipiIsrSpec> {
        IsrErrCsW::new(self, 23)
    }
    #[doc = "Bit 24 - 1: set register; 0: nothing happens"]
    #[inline(always)]
    #[must_use]
    pub fn isr_frame_end(&mut self) -> IsrFrameEndW<MipiIsrSpec> {
        IsrFrameEndW::new(self, 24)
    }
    #[doc = "Bit 25 - 1: set register; 0: nothing happens"]
    #[inline(always)]
    #[must_use]
    pub fn isr_add_data_ovflw(&mut self) -> IsrAddDataOvflwW<MipiIsrSpec> {
        IsrAddDataOvflwW::new(self, 25)
    }
    #[doc = "Bit 27 - 1: set register; 0: nothing happens"]
    #[inline(always)]
    #[must_use]
    pub fn isr_gen_short_pack(&mut self) -> IsrGenShortPackW<MipiIsrSpec> {
        IsrGenShortPackW::new(self, 27)
    }
}
#[doc = "Interrupt set register\n\nNote: sets corresponding bits in MIPI_RIS register \n\n\n\n \n\n\n\n\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_isr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiIsrSpec;
impl crate::RegisterSpec for MipiIsrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mipi_isr::W`](W) writer structure"]
impl crate::Writable for MipiIsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIPI_ISR to value 0"]
impl crate::Resettable for MipiIsrSpec {
    const RESET_VALUE: u32 = 0;
}
