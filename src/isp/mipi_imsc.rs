#[doc = "Register `MIPI_IMSC` reader"]
pub type R = crate::R<MipiImscSpec>;
#[doc = "Register `MIPI_IMSC` writer"]
pub type W = crate::W<MipiImscSpec>;
#[doc = "Field `IMSC_ERR_SOT` reader - enable interrupt (1) or mask out (0)"]
pub type ImscErrSotR = crate::FieldReader;
#[doc = "Field `IMSC_ERR_SOT` writer - enable interrupt (1) or mask out (0)"]
pub type ImscErrSotW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IMSC_ERR_SOT_SYNC` reader - enable interrupt (1) or mask out (0)"]
pub type ImscErrSotSyncR = crate::FieldReader;
#[doc = "Field `IMSC_ERR_SOT_SYNC` writer - enable interrupt (1) or mask out (0)"]
pub type ImscErrSotSyncW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IMSC_ERR_EOT_SYNC` reader - enable interrupt (1) or mask out (0)"]
pub type ImscErrEotSyncR = crate::FieldReader;
#[doc = "Field `IMSC_ERR_EOT_SYNC` writer - enable interrupt (1) or mask out (0)"]
pub type ImscErrEotSyncW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IMSC_ERR_CONTROL` reader - enable interrupt (1) or mask out (0)"]
pub type ImscErrControlR = crate::FieldReader;
#[doc = "Field `IMSC_ERR_CONTROL` writer - enable interrupt (1) or mask out (0)"]
pub type ImscErrControlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IMSC_ERR_PROTOCOL` reader - enable interrupt (1) or mask out (0)"]
pub type ImscErrProtocolR = crate::BitReader;
#[doc = "Field `IMSC_ERR_PROTOCOL` writer - enable interrupt (1) or mask out (0)"]
pub type ImscErrProtocolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_ERR_ECC2` reader - enable interrupt (1) or mask out (0)"]
pub type ImscErrEcc2R = crate::BitReader;
#[doc = "Field `IMSC_ERR_ECC2` writer - enable interrupt (1) or mask out (0)"]
pub type ImscErrEcc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_ERR_ECC1` reader - enable interrupt (1) or mask out (0)"]
pub type ImscErrEcc1R = crate::BitReader;
#[doc = "Field `IMSC_ERR_ECC1` writer - enable interrupt (1) or mask out (0)"]
pub type ImscErrEcc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_ERR_CS` reader - enable interrupt (1) or mask out (0)"]
pub type ImscErrCsR = crate::BitReader;
#[doc = "Field `IMSC_ERR_CS` writer - enable interrupt (1) or mask out (0)"]
pub type ImscErrCsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_FRAME_END` reader - enable interrupt (1) or mask out (0)"]
pub type ImscFrameEndR = crate::BitReader;
#[doc = "Field `IMSC_FRAME_END` writer - enable interrupt (1) or mask out (0)"]
pub type ImscFrameEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_ADD_DATA_OVFLW` reader - enable interrupt (1) or mask out (0)"]
pub type ImscAddDataOvflwR = crate::BitReader;
#[doc = "Field `IMSC_ADD_DATA_OVFLW` writer - enable interrupt (1) or mask out (0)"]
pub type ImscAddDataOvflwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_GEN_SHORT_PACK` reader - enable interrupt (1) or mask out (0)"]
pub type ImscGenShortPackR = crate::BitReader;
#[doc = "Field `IMSC_GEN_SHORT_PACK` writer - enable interrupt (1) or mask out (0)"]
pub type ImscGenShortPackW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:7 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_err_sot(&self) -> ImscErrSotR {
        ImscErrSotR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_err_sot_sync(&self) -> ImscErrSotSyncR {
        ImscErrSotSyncR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_err_eot_sync(&self) -> ImscErrEotSyncR {
        ImscErrEotSyncR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_err_control(&self) -> ImscErrControlR {
        ImscErrControlR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_err_protocol(&self) -> ImscErrProtocolR {
        ImscErrProtocolR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_err_ecc2(&self) -> ImscErrEcc2R {
        ImscErrEcc2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_err_ecc1(&self) -> ImscErrEcc1R {
        ImscErrEcc1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_err_cs(&self) -> ImscErrCsR {
        ImscErrCsR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_frame_end(&self) -> ImscFrameEndR {
        ImscFrameEndR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_add_data_ovflw(&self) -> ImscAddDataOvflwR {
        ImscAddDataOvflwR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_gen_short_pack(&self) -> ImscGenShortPackR {
        ImscGenShortPackR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_err_sot(&mut self) -> ImscErrSotW<MipiImscSpec> {
        ImscErrSotW::new(self, 4)
    }
    #[doc = "Bits 8:11 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_err_sot_sync(&mut self) -> ImscErrSotSyncW<MipiImscSpec> {
        ImscErrSotSyncW::new(self, 8)
    }
    #[doc = "Bits 12:15 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_err_eot_sync(&mut self) -> ImscErrEotSyncW<MipiImscSpec> {
        ImscErrEotSyncW::new(self, 12)
    }
    #[doc = "Bits 16:19 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_err_control(&mut self) -> ImscErrControlW<MipiImscSpec> {
        ImscErrControlW::new(self, 16)
    }
    #[doc = "Bit 20 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_err_protocol(&mut self) -> ImscErrProtocolW<MipiImscSpec> {
        ImscErrProtocolW::new(self, 20)
    }
    #[doc = "Bit 21 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_err_ecc2(&mut self) -> ImscErrEcc2W<MipiImscSpec> {
        ImscErrEcc2W::new(self, 21)
    }
    #[doc = "Bit 22 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_err_ecc1(&mut self) -> ImscErrEcc1W<MipiImscSpec> {
        ImscErrEcc1W::new(self, 22)
    }
    #[doc = "Bit 23 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_err_cs(&mut self) -> ImscErrCsW<MipiImscSpec> {
        ImscErrCsW::new(self, 23)
    }
    #[doc = "Bit 24 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_frame_end(&mut self) -> ImscFrameEndW<MipiImscSpec> {
        ImscFrameEndW::new(self, 24)
    }
    #[doc = "Bit 25 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_add_data_ovflw(&mut self) -> ImscAddDataOvflwW<MipiImscSpec> {
        ImscAddDataOvflwW::new(self, 25)
    }
    #[doc = "Bit 27 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_gen_short_pack(&mut self) -> ImscGenShortPackW<MipiImscSpec> {
        ImscGenShortPackW::new(self, 27)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_imsc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_imsc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiImscSpec;
impl crate::RegisterSpec for MipiImscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_imsc::R`](R) reader structure"]
impl crate::Readable for MipiImscSpec {}
#[doc = "`write(|w| ..)` method takes [`mipi_imsc::W`](W) writer structure"]
impl crate::Writable for MipiImscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIPI_IMSC to value 0"]
impl crate::Resettable for MipiImscSpec {
    const RESET_VALUE: u32 = 0;
}
