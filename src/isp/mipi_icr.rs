#[doc = "Register `MIPI_ICR` writer"]
pub type W = crate::W<MipiIcrSpec>;
#[doc = "Field `ICR_SYNC_FIFO_OVFLW` writer - 1: clear register; 0: nothing happens (one bit for each\n\nlane)"]
pub type IcrSyncFifoOvflwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ICR_ERR_SOT` writer - 1: clear register; 0: nothing happens (one bit for each\n\nlane)\n\n"]
pub type IcrErrSotW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ICR_ERR_SOT_SYNC` writer - 1: clear register; 0: nothing happens (one bit for each\n\nlane)"]
pub type IcrErrSotSyncW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IcrErrEotSync {
    #[doc = "1: clear register;"]
    B1 = 1,
    #[doc = "0: nothing happens (one bit for each lane)"]
    B0 = 0,
}
impl From<IcrErrEotSync> for u8 {
    #[inline(always)]
    fn from(variant: IcrErrEotSync) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IcrErrEotSync {
    type Ux = u8;
}
#[doc = "Field `ICR_ERR_EOT_SYNC` writer - "]
pub type IcrErrEotSyncW<'a, REG> = crate::FieldWriter<'a, REG, 4, IcrErrEotSync>;
impl<'a, REG> IcrErrEotSyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "clear register;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IcrErrEotSync::B1)
    }
    #[doc = "nothing happens (one bit for each lane)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IcrErrEotSync::B0)
    }
}
#[doc = "Field `ICR_ERR_CONTROL` writer - 1: clear register; 0: nothing happens (one bit for each\n\nlane)\n\n\n\n"]
pub type IcrErrControlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ICR_ERR_PROTOCOL` writer - 1: clear register; 0: nothing happens"]
pub type IcrErrProtocolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_ERR_ECC2` writer - 1: clear register; 0: nothing happens"]
pub type IcrErrEcc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_ERR_ECC1` writer - 1: clear register; 0: nothing happens"]
pub type IcrErrEcc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_ERR_CS` writer - 1: clear register; 0: nothing happens"]
pub type IcrErrCsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_FRAME_END` writer - 1: clear register; 0: nothing happens"]
pub type IcrFrameEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_ADD_DATA_OVFLW` writer - 1: clear register; 0: nothing happens"]
pub type IcrAddDataOvflwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_GEN_SHORT_PACK` writer - 1: clear register; 0: nothing happens"]
pub type IcrGenShortPackW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:3 - 1: clear register; 0: nothing happens (one bit for each\n\nlane)"]
    #[inline(always)]
    #[must_use]
    pub fn icr_sync_fifo_ovflw(&mut self) -> IcrSyncFifoOvflwW<MipiIcrSpec> {
        IcrSyncFifoOvflwW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 1: clear register; 0: nothing happens (one bit for each\n\nlane)\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn icr_err_sot(&mut self) -> IcrErrSotW<MipiIcrSpec> {
        IcrErrSotW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 1: clear register; 0: nothing happens (one bit for each\n\nlane)"]
    #[inline(always)]
    #[must_use]
    pub fn icr_err_sot_sync(&mut self) -> IcrErrSotSyncW<MipiIcrSpec> {
        IcrErrSotSyncW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn icr_err_eot_sync(&mut self) -> IcrErrEotSyncW<MipiIcrSpec> {
        IcrErrEotSyncW::new(self, 12)
    }
    #[doc = "Bits 16:19 - 1: clear register; 0: nothing happens (one bit for each\n\nlane)\n\n\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn icr_err_control(&mut self) -> IcrErrControlW<MipiIcrSpec> {
        IcrErrControlW::new(self, 16)
    }
    #[doc = "Bit 20 - 1: clear register; 0: nothing happens"]
    #[inline(always)]
    #[must_use]
    pub fn icr_err_protocol(&mut self) -> IcrErrProtocolW<MipiIcrSpec> {
        IcrErrProtocolW::new(self, 20)
    }
    #[doc = "Bit 21 - 1: clear register; 0: nothing happens"]
    #[inline(always)]
    #[must_use]
    pub fn icr_err_ecc2(&mut self) -> IcrErrEcc2W<MipiIcrSpec> {
        IcrErrEcc2W::new(self, 21)
    }
    #[doc = "Bit 22 - 1: clear register; 0: nothing happens"]
    #[inline(always)]
    #[must_use]
    pub fn icr_err_ecc1(&mut self) -> IcrErrEcc1W<MipiIcrSpec> {
        IcrErrEcc1W::new(self, 22)
    }
    #[doc = "Bit 23 - 1: clear register; 0: nothing happens"]
    #[inline(always)]
    #[must_use]
    pub fn icr_err_cs(&mut self) -> IcrErrCsW<MipiIcrSpec> {
        IcrErrCsW::new(self, 23)
    }
    #[doc = "Bit 24 - 1: clear register; 0: nothing happens"]
    #[inline(always)]
    #[must_use]
    pub fn icr_frame_end(&mut self) -> IcrFrameEndW<MipiIcrSpec> {
        IcrFrameEndW::new(self, 24)
    }
    #[doc = "Bit 25 - 1: clear register; 0: nothing happens"]
    #[inline(always)]
    #[must_use]
    pub fn icr_add_data_ovflw(&mut self) -> IcrAddDataOvflwW<MipiIcrSpec> {
        IcrAddDataOvflwW::new(self, 25)
    }
    #[doc = "Bit 27 - 1: clear register; 0: nothing happens"]
    #[inline(always)]
    #[must_use]
    pub fn icr_gen_short_pack(&mut self) -> IcrGenShortPackW<MipiIcrSpec> {
        IcrGenShortPackW::new(self, 27)
    }
}
#[doc = "Interrupt clear register\n\nNote: clears corresponding bits in MIPI_RIS register \n\n\n\n \n\n\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiIcrSpec;
impl crate::RegisterSpec for MipiIcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mipi_icr::W`](W) writer structure"]
impl crate::Writable for MipiIcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIPI_ICR to value 0"]
impl crate::Resettable for MipiIcrSpec {
    const RESET_VALUE: u32 = 0;
}
