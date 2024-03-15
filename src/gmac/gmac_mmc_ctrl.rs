#[doc = "Register `GMAC_MMC_CTRL` reader"]
pub type R = crate::R<GmacMmcCtrlSpec>;
#[doc = "Register `GMAC_MMC_CTRL` writer"]
pub type W = crate::W<GmacMmcCtrlSpec>;
#[doc = "Field `CR` reader - Counters Reset When set, all counters will be reset. This bit will be cleared automatically after 1 clock cycle"]
pub type CrR = crate::BitReader;
#[doc = "Field `CR` writer - Counters Reset When set, all counters will be reset. This bit will be cleared automatically after 1 clock cycle"]
pub type CrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSR` reader - Counter Stop Rollover When set, counter after reaching maximum value will not roll over to zero"]
pub type CsrR = crate::BitReader;
#[doc = "Field `CSR` writer - Counter Stop Rollover When set, counter after reaching maximum value will not roll over to zero"]
pub type CsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROR` reader - Reset on Read When set, the MMC counters will be reset to zero after Read (self- clearing after reset). The counters are cleared when the least significant byte lane (bits\\[7:0\\]) is read."]
pub type RorR = crate::BitReader;
#[doc = "Field `ROR` writer - Reset on Read When set, the MMC counters will be reset to zero after Read (self- clearing after reset). The counters are cleared when the least significant byte lane (bits\\[7:0\\]) is read."]
pub type RorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCF` reader - MMC Counter Freeze When set, this bit freezes all the MMC counters to their current value. (None of the MMC counters are updated due to any transmitted or received frame until this bit is reset to 0. If any MMC counter is read with the Reset on Read bit set, then that counter is also cleared in this mode.)"]
pub type McfR = crate::BitReader;
#[doc = "Field `MCF` writer - MMC Counter Freeze When set, this bit freezes all the MMC counters to their current value. (None of the MMC counters are updated due to any transmitted or received frame until this bit is reset to 0. If any MMC counter is read with the Reset on Read bit set, then that counter is also cleared in this mode.)"]
pub type McfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP` reader - Counters Preset When set, all counters will be initialized or preset to almost full or almost half as per Bit5 above. This bit will be cleared automatically after 1 clock cycle. This bit along with bit5 is useful for debugging and testing the assertion of interrupts due to MMC counter becoming half-full or full."]
pub type CpR = crate::BitReader;
#[doc = "Field `CP` writer - Counters Preset When set, all counters will be initialized or preset to almost full or almost half as per Bit5 above. This bit will be cleared automatically after 1 clock cycle. This bit along with bit5 is useful for debugging and testing the assertion of interrupts due to MMC counter becoming half-full or full."]
pub type CpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FHP` reader - Full-Half preset When low and bit4 is set, all MMC counters get preset to almost- half value. All octet counters get preset to 0x7FFF_F800 (half - 2K Bytes) and all frame-counters gets preset to 0x7FFF_FFF0 (half - 16) When high and bit4 is set, all MMC counters get preset to almost- full value. All octet counters get preset to 0xFFFF_F800 (full - 2K Bytes) and all frame-counters gets preset to 0xFFFF_FFF0 (full - 16)"]
pub type FhpR = crate::BitReader;
#[doc = "Field `FHP` writer - Full-Half preset When low and bit4 is set, all MMC counters get preset to almost- half value. All octet counters get preset to 0x7FFF_F800 (half - 2K Bytes) and all frame-counters gets preset to 0x7FFF_FFF0 (half - 16) When high and bit4 is set, all MMC counters get preset to almost- full value. All octet counters get preset to 0xFFFF_F800 (full - 2K Bytes) and all frame-counters gets preset to 0xFFFF_FFF0 (full - 16)"]
pub type FhpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Counters Reset When set, all counters will be reset. This bit will be cleared automatically after 1 clock cycle"]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        CrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter Stop Rollover When set, counter after reaching maximum value will not roll over to zero"]
    #[inline(always)]
    pub fn csr(&self) -> CsrR {
        CsrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset on Read When set, the MMC counters will be reset to zero after Read (self- clearing after reset). The counters are cleared when the least significant byte lane (bits\\[7:0\\]) is read."]
    #[inline(always)]
    pub fn ror(&self) -> RorR {
        RorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Counter Freeze When set, this bit freezes all the MMC counters to their current value. (None of the MMC counters are updated due to any transmitted or received frame until this bit is reset to 0. If any MMC counter is read with the Reset on Read bit set, then that counter is also cleared in this mode.)"]
    #[inline(always)]
    pub fn mcf(&self) -> McfR {
        McfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Counters Preset When set, all counters will be initialized or preset to almost full or almost half as per Bit5 above. This bit will be cleared automatically after 1 clock cycle. This bit along with bit5 is useful for debugging and testing the assertion of interrupts due to MMC counter becoming half-full or full."]
    #[inline(always)]
    pub fn cp(&self) -> CpR {
        CpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Full-Half preset When low and bit4 is set, all MMC counters get preset to almost- half value. All octet counters get preset to 0x7FFF_F800 (half - 2K Bytes) and all frame-counters gets preset to 0x7FFF_FFF0 (half - 16) When high and bit4 is set, all MMC counters get preset to almost- full value. All octet counters get preset to 0xFFFF_F800 (full - 2K Bytes) and all frame-counters gets preset to 0xFFFF_FFF0 (full - 16)"]
    #[inline(always)]
    pub fn fhp(&self) -> FhpR {
        FhpR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counters Reset When set, all counters will be reset. This bit will be cleared automatically after 1 clock cycle"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CrW<GmacMmcCtrlSpec> {
        CrW::new(self, 0)
    }
    #[doc = "Bit 1 - Counter Stop Rollover When set, counter after reaching maximum value will not roll over to zero"]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CsrW<GmacMmcCtrlSpec> {
        CsrW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset on Read When set, the MMC counters will be reset to zero after Read (self- clearing after reset). The counters are cleared when the least significant byte lane (bits\\[7:0\\]) is read."]
    #[inline(always)]
    #[must_use]
    pub fn ror(&mut self) -> RorW<GmacMmcCtrlSpec> {
        RorW::new(self, 2)
    }
    #[doc = "Bit 3 - MMC Counter Freeze When set, this bit freezes all the MMC counters to their current value. (None of the MMC counters are updated due to any transmitted or received frame until this bit is reset to 0. If any MMC counter is read with the Reset on Read bit set, then that counter is also cleared in this mode.)"]
    #[inline(always)]
    #[must_use]
    pub fn mcf(&mut self) -> McfW<GmacMmcCtrlSpec> {
        McfW::new(self, 3)
    }
    #[doc = "Bit 4 - Counters Preset When set, all counters will be initialized or preset to almost full or almost half as per Bit5 above. This bit will be cleared automatically after 1 clock cycle. This bit along with bit5 is useful for debugging and testing the assertion of interrupts due to MMC counter becoming half-full or full."]
    #[inline(always)]
    #[must_use]
    pub fn cp(&mut self) -> CpW<GmacMmcCtrlSpec> {
        CpW::new(self, 4)
    }
    #[doc = "Bit 5 - Full-Half preset When low and bit4 is set, all MMC counters get preset to almost- half value. All octet counters get preset to 0x7FFF_F800 (half - 2K Bytes) and all frame-counters gets preset to 0x7FFF_FFF0 (half - 16) When high and bit4 is set, all MMC counters get preset to almost- full value. All octet counters get preset to 0xFFFF_F800 (full - 2K Bytes) and all frame-counters gets preset to 0xFFFF_FFF0 (full - 16)"]
    #[inline(always)]
    #[must_use]
    pub fn fhp(&mut self) -> FhpW<GmacMmcCtrlSpec> {
        FhpW::new(self, 5)
    }
}
#[doc = "MMC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMmcCtrlSpec;
impl crate::RegisterSpec for GmacMmcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mmc_ctrl::R`](R) reader structure"]
impl crate::Readable for GmacMmcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mmc_ctrl::W`](W) writer structure"]
impl crate::Writable for GmacMmcCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MMC_CTRL to value 0"]
impl crate::Resettable for GmacMmcCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
