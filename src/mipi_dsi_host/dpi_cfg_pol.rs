#[doc = "Register `DPI_CFG_POL` reader"]
pub type R = crate::R<DpiCfgPolSpec>;
#[doc = "Register `DPI_CFG_POL` writer"]
pub type W = crate::W<DpiCfgPolSpec>;
#[doc = "Field `DATAEN_ACTIVE_LOW` reader - Field0000 Abstract\n\nWhen set to 1, this bit configures the data enable pin (dpidataen) as\n\nactive low."]
pub type DataenActiveLowR = crate::BitReader;
#[doc = "Field `DATAEN_ACTIVE_LOW` writer - Field0000 Abstract\n\nWhen set to 1, this bit configures the data enable pin (dpidataen) as\n\nactive low."]
pub type DataenActiveLowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSYNC_ACTIVE_LOW` reader - Field0001 Abstract\n\nWhen set to 1, this bit configures the vertical synchronism pin\n\n(dpivsync) as active low."]
pub type VsyncActiveLowR = crate::BitReader;
#[doc = "Field `VSYNC_ACTIVE_LOW` writer - Field0001 Abstract\n\nWhen set to 1, this bit configures the vertical synchronism pin\n\n(dpivsync) as active low."]
pub type VsyncActiveLowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSYNC_ACTIVE_LOW` reader - Field0001 Abstract\n\nWhen set to 1, this bit configures the horizontal synchronism pin\n\n(dpihsync) as active low."]
pub type HsyncActiveLowR = crate::BitReader;
#[doc = "Field `HSYNC_ACTIVE_LOW` writer - Field0001 Abstract\n\nWhen set to 1, this bit configures the horizontal synchronism pin\n\n(dpihsync) as active low."]
pub type HsyncActiveLowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHUTD_ACTIVE_LOW` reader - Field0001 Abstract\n\nWhen set to 1, this bit configures the shutdown pin (dpishutdn) as\n\nactive low."]
pub type ShutdActiveLowR = crate::BitReader;
#[doc = "Field `SHUTD_ACTIVE_LOW` writer - Field0001 Abstract\n\nWhen set to 1, this bit configures the shutdown pin (dpishutdn) as\n\nactive low."]
pub type ShutdActiveLowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLORM_ACTIVE_LOW` reader - Field0001 Abstract\n\nWhen set to 1, this bit configures the color mode pin (dpicolorm) as\n\nactive low."]
pub type ColormActiveLowR = crate::BitReader;
#[doc = "Field `COLORM_ACTIVE_LOW` writer - Field0001 Abstract\n\nWhen set to 1, this bit configures the color mode pin (dpicolorm) as\n\nactive low."]
pub type ColormActiveLowW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Field0000 Abstract\n\nWhen set to 1, this bit configures the data enable pin (dpidataen) as\n\nactive low."]
    #[inline(always)]
    pub fn dataen_active_low(&self) -> DataenActiveLowR {
        DataenActiveLowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Field0001 Abstract\n\nWhen set to 1, this bit configures the vertical synchronism pin\n\n(dpivsync) as active low."]
    #[inline(always)]
    pub fn vsync_active_low(&self) -> VsyncActiveLowR {
        VsyncActiveLowR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Field0001 Abstract\n\nWhen set to 1, this bit configures the horizontal synchronism pin\n\n(dpihsync) as active low."]
    #[inline(always)]
    pub fn hsync_active_low(&self) -> HsyncActiveLowR {
        HsyncActiveLowR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Field0001 Abstract\n\nWhen set to 1, this bit configures the shutdown pin (dpishutdn) as\n\nactive low."]
    #[inline(always)]
    pub fn shutd_active_low(&self) -> ShutdActiveLowR {
        ShutdActiveLowR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Field0001 Abstract\n\nWhen set to 1, this bit configures the color mode pin (dpicolorm) as\n\nactive low."]
    #[inline(always)]
    pub fn colorm_active_low(&self) -> ColormActiveLowR {
        ColormActiveLowR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Field0000 Abstract\n\nWhen set to 1, this bit configures the data enable pin (dpidataen) as\n\nactive low."]
    #[inline(always)]
    #[must_use]
    pub fn dataen_active_low(&mut self) -> DataenActiveLowW<DpiCfgPolSpec> {
        DataenActiveLowW::new(self, 0)
    }
    #[doc = "Bit 1 - Field0001 Abstract\n\nWhen set to 1, this bit configures the vertical synchronism pin\n\n(dpivsync) as active low."]
    #[inline(always)]
    #[must_use]
    pub fn vsync_active_low(&mut self) -> VsyncActiveLowW<DpiCfgPolSpec> {
        VsyncActiveLowW::new(self, 1)
    }
    #[doc = "Bit 2 - Field0001 Abstract\n\nWhen set to 1, this bit configures the horizontal synchronism pin\n\n(dpihsync) as active low."]
    #[inline(always)]
    #[must_use]
    pub fn hsync_active_low(&mut self) -> HsyncActiveLowW<DpiCfgPolSpec> {
        HsyncActiveLowW::new(self, 2)
    }
    #[doc = "Bit 3 - Field0001 Abstract\n\nWhen set to 1, this bit configures the shutdown pin (dpishutdn) as\n\nactive low."]
    #[inline(always)]
    #[must_use]
    pub fn shutd_active_low(&mut self) -> ShutdActiveLowW<DpiCfgPolSpec> {
        ShutdActiveLowW::new(self, 3)
    }
    #[doc = "Bit 4 - Field0001 Abstract\n\nWhen set to 1, this bit configures the color mode pin (dpicolorm) as\n\nactive low."]
    #[inline(always)]
    #[must_use]
    pub fn colorm_active_low(&mut self) -> ColormActiveLowW<DpiCfgPolSpec> {
        ColormActiveLowW::new(self, 4)
    }
}
#[doc = "DPI Polarity Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi_cfg_pol::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_cfg_pol::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpiCfgPolSpec;
impl crate::RegisterSpec for DpiCfgPolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_cfg_pol::R`](R) reader structure"]
impl crate::Readable for DpiCfgPolSpec {}
#[doc = "`write(|w| ..)` method takes [`dpi_cfg_pol::W`](W) writer structure"]
impl crate::Writable for DpiCfgPolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPI_CFG_POL to value 0"]
impl crate::Resettable for DpiCfgPolSpec {
    const RESET_VALUE: u32 = 0;
}
