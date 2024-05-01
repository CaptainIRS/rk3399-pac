#[doc = "Register `DPI_VCID` reader"]
pub type R = crate::R<DpiVcidSpec>;
#[doc = "Register `DPI_VCID` writer"]
pub type W = crate::W<DpiVcidSpec>;
#[doc = "Field `DPI_VCID` reader - dpi_vcid\n\nThis field configures the DPI virtual channel id that is indexed to the\n\nVideo mode packets."]
pub type DpiVcidR = crate::FieldReader;
#[doc = "Field `DPI_VCID` writer - dpi_vcid\n\nThis field configures the DPI virtual channel id that is indexed to the\n\nVideo mode packets."]
pub type DpiVcidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - dpi_vcid\n\nThis field configures the DPI virtual channel id that is indexed to the\n\nVideo mode packets."]
    #[inline(always)]
    pub fn dpi_vcid(&self) -> DpiVcidR {
        DpiVcidR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - dpi_vcid\n\nThis field configures the DPI virtual channel id that is indexed to the\n\nVideo mode packets."]
    #[inline(always)]
    #[must_use]
    pub fn dpi_vcid(&mut self) -> DpiVcidW<DpiVcidSpec> {
        DpiVcidW::new(self, 0)
    }
}
#[doc = "DPI Virtual Channel ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi_vcid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_vcid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpiVcidSpec;
impl crate::RegisterSpec for DpiVcidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_vcid::R`](R) reader structure"]
impl crate::Readable for DpiVcidSpec {}
#[doc = "`write(|w| ..)` method takes [`dpi_vcid::W`](W) writer structure"]
impl crate::Writable for DpiVcidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPI_VCID to value 0"]
impl crate::Resettable for DpiVcidSpec {
    const RESET_VALUE: u32 = 0;
}
