#[doc = "Register `COMMON_INT_STA_1` reader"]
pub type R = crate::R<CommonIntSta1Spec>;
#[doc = "Register `COMMON_INT_STA_1` writer"]
pub type W = crate::W<CommonIntSta1Spec>;
#[doc = "Field `SW_INT` reader - 1: Software-induced interrupt. Write 1 to clear."]
pub type SwIntR = crate::BitReader;
#[doc = "Field `SW_INT` writer - 1: Software-induced interrupt. Write 1 to clear."]
pub type SwIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VID_CLK_CHG` reader - 1: Video input clock change is detected."]
pub type VidClkChgR = crate::BitReader;
#[doc = "Field `VID_CLK_CHG` writer - 1: Video input clock change is detected."]
pub type VidClkChgW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PSR_VID_CRC_VALID` reader - 1: PSR video CRC value is valid."]
pub type PsrVidCrcValidR = crate::BitReader;
#[doc = "Field `PSR_VID_CRC_VALID` writer - 1: PSR video CRC value is valid."]
pub type PsrVidCrcValidW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `VID_FORMAT_CHG` reader - 1: Video input format change is detected. Write 1 to clear."]
pub type VidFormatChgR = crate::BitReader;
#[doc = "Field `VID_FORMAT_CHG` writer - 1: Video input format change is detected. Write 1 to clear."]
pub type VidFormatChgW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PLL_LOCK_CHG` reader - 1: PLL lock state is changed. Write 1 to clear. Check PLL_LOCK of register DP_DEBUG_CTL for PLL lock status."]
pub type PllLockChgR = crate::BitReader;
#[doc = "Field `PLL_LOCK_CHG` writer - 1: PLL lock state is changed. Write 1 to clear. Check PLL_LOCK of register DP_DEBUG_CTL for PLL lock status."]
pub type PllLockChgW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `VSYNC_DET` reader - 1: VSYNC active edge has been detected. Write 1 to clear."]
pub type VsyncDetR = crate::BitReader;
#[doc = "Field `VSYNC_DET` writer - 1: VSYNC active edge has been detected. Write 1 to clear."]
pub type VsyncDetW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Software-induced interrupt. Write 1 to clear."]
    #[inline(always)]
    pub fn sw_int(&self) -> SwIntR {
        SwIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Video input clock change is detected."]
    #[inline(always)]
    pub fn vid_clk_chg(&self) -> VidClkChgR {
        VidClkChgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: PSR video CRC value is valid."]
    #[inline(always)]
    pub fn psr_vid_crc_valid(&self) -> PsrVidCrcValidR {
        PsrVidCrcValidR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Video input format change is detected. Write 1 to clear."]
    #[inline(always)]
    pub fn vid_format_chg(&self) -> VidFormatChgR {
        VidFormatChgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: PLL lock state is changed. Write 1 to clear. Check PLL_LOCK of register DP_DEBUG_CTL for PLL lock status."]
    #[inline(always)]
    pub fn pll_lock_chg(&self) -> PllLockChgR {
        PllLockChgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: VSYNC active edge has been detected. Write 1 to clear."]
    #[inline(always)]
    pub fn vsync_det(&self) -> VsyncDetR {
        VsyncDetR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Software-induced interrupt. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn sw_int(&mut self) -> SwIntW<CommonIntSta1Spec> {
        SwIntW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Video input clock change is detected."]
    #[inline(always)]
    #[must_use]
    pub fn vid_clk_chg(&mut self) -> VidClkChgW<CommonIntSta1Spec> {
        VidClkChgW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: PSR video CRC value is valid."]
    #[inline(always)]
    #[must_use]
    pub fn psr_vid_crc_valid(&mut self) -> PsrVidCrcValidW<CommonIntSta1Spec> {
        PsrVidCrcValidW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Video input format change is detected. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn vid_format_chg(&mut self) -> VidFormatChgW<CommonIntSta1Spec> {
        VidFormatChgW::new(self, 3)
    }
    #[doc = "Bit 6 - 1: PLL lock state is changed. Write 1 to clear. Check PLL_LOCK of register DP_DEBUG_CTL for PLL lock status."]
    #[inline(always)]
    #[must_use]
    pub fn pll_lock_chg(&mut self) -> PllLockChgW<CommonIntSta1Spec> {
        PllLockChgW::new(self, 6)
    }
    #[doc = "Bit 7 - 1: VSYNC active edge has been detected. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn vsync_det(&mut self) -> VsyncDetW<CommonIntSta1Spec> {
        VsyncDetW::new(self, 7)
    }
}
#[doc = "Common Interrupt Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`common_int_sta_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`common_int_sta_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommonIntSta1Spec;
impl crate::RegisterSpec for CommonIntSta1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`common_int_sta_1::R`](R) reader structure"]
impl crate::Readable for CommonIntSta1Spec {}
#[doc = "`write(|w| ..)` method takes [`common_int_sta_1::W`](W) writer structure"]
impl crate::Writable for CommonIntSta1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xce;
}
#[doc = "`reset()` method sets COMMON_INT_STA_1 to value 0"]
impl crate::Resettable for CommonIntSta1Spec {
    const RESET_VALUE: u32 = 0;
}
