#[doc = "Register `FC_AVICONF0` reader"]
pub type R = crate::R<FcAviconf0Spec>;
#[doc = "Register `FC_AVICONF0` writer"]
pub type W = crate::W<FcAviconf0Spec>;
#[doc = "Field `RGC_YCC_INDICATION` reader - Y1,Y0 RGB or YCC indicator"]
pub type RgcYccIndicationR = crate::FieldReader;
#[doc = "Field `RGC_YCC_INDICATION` writer - Y1,Y0 RGB or YCC indicator"]
pub type RgcYccIndicationW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BAR_INFORMATION` reader - Bar information data valid"]
pub type BarInformationR = crate::FieldReader;
#[doc = "Field `BAR_INFORMATION` writer - Bar information data valid"]
pub type BarInformationW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_INFORMATION` reader - Scan information"]
pub type ScanInformationR = crate::FieldReader;
#[doc = "Field `SCAN_INFORMATION` writer - Scan information"]
pub type ScanInformationW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ACTIVE_FORMAT_PRESENT` reader - Active format present"]
pub type ActiveFormatPresentR = crate::BitReader;
#[doc = "Field `ACTIVE_FORMAT_PRESENT` writer - Active format present"]
pub type ActiveFormatPresentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGC_YCC_INDICATION_2` reader - Y2, Bit 2 of rgc_ycc_indication"]
pub type RgcYccIndication2R = crate::BitReader;
#[doc = "Field `RGC_YCC_INDICATION_2` writer - Y2, Bit 2 of rgc_ycc_indication"]
pub type RgcYccIndication2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Y1,Y0 RGB or YCC indicator"]
    #[inline(always)]
    pub fn rgc_ycc_indication(&self) -> RgcYccIndicationR {
        RgcYccIndicationR::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Bar information data valid"]
    #[inline(always)]
    pub fn bar_information(&self) -> BarInformationR {
        BarInformationR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Scan information"]
    #[inline(always)]
    pub fn scan_information(&self) -> ScanInformationR {
        ScanInformationR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Active format present"]
    #[inline(always)]
    pub fn active_format_present(&self) -> ActiveFormatPresentR {
        ActiveFormatPresentR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Y2, Bit 2 of rgc_ycc_indication"]
    #[inline(always)]
    pub fn rgc_ycc_indication_2(&self) -> RgcYccIndication2R {
        RgcYccIndication2R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Y1,Y0 RGB or YCC indicator"]
    #[inline(always)]
    #[must_use]
    pub fn rgc_ycc_indication(&mut self) -> RgcYccIndicationW<FcAviconf0Spec> {
        RgcYccIndicationW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Bar information data valid"]
    #[inline(always)]
    #[must_use]
    pub fn bar_information(&mut self) -> BarInformationW<FcAviconf0Spec> {
        BarInformationW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Scan information"]
    #[inline(always)]
    #[must_use]
    pub fn scan_information(&mut self) -> ScanInformationW<FcAviconf0Spec> {
        ScanInformationW::new(self, 4)
    }
    #[doc = "Bit 6 - Active format present"]
    #[inline(always)]
    #[must_use]
    pub fn active_format_present(&mut self) -> ActiveFormatPresentW<FcAviconf0Spec> {
        ActiveFormatPresentW::new(self, 6)
    }
    #[doc = "Bit 7 - Y2, Bit 2 of rgc_ycc_indication"]
    #[inline(always)]
    #[must_use]
    pub fn rgc_ycc_indication_2(&mut self) -> RgcYccIndication2W<FcAviconf0Spec> {
        RgcYccIndication2W::new(self, 7)
    }
}
#[doc = "Y1,Y0 RGB or YCC indicator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_aviconf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_aviconf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAviconf0Spec;
impl crate::RegisterSpec for FcAviconf0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_aviconf0::R`](R) reader structure"]
impl crate::Readable for FcAviconf0Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_aviconf0::W`](W) writer structure"]
impl crate::Writable for FcAviconf0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AVICONF0 to value 0"]
impl crate::Resettable for FcAviconf0Spec {
    const RESET_VALUE: u8 = 0;
}
