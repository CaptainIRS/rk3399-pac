#[doc = "Register `GMII_DATA` reader"]
pub type R = crate::R<GmiiDataSpec>;
#[doc = "Register `GMII_DATA` writer"]
pub type W = crate::W<GmiiDataSpec>;
#[doc = "Field `GD` reader - GMII Data\n\nThis contains the 16-bit data value read from the PHY after a\n\nManagement Read operation or the 16-bit data value to be\n\nwritten to the PHY before a Management Write operation."]
pub type GdR = crate::FieldReader<u16>;
#[doc = "Field `GD` writer - GMII Data\n\nThis contains the 16-bit data value read from the PHY after a\n\nManagement Read operation or the 16-bit data value to be\n\nwritten to the PHY before a Management Write operation."]
pub type GdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GMII Data\n\nThis contains the 16-bit data value read from the PHY after a\n\nManagement Read operation or the 16-bit data value to be\n\nwritten to the PHY before a Management Write operation."]
    #[inline(always)]
    pub fn gd(&self) -> GdR {
        GdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GMII Data\n\nThis contains the 16-bit data value read from the PHY after a\n\nManagement Read operation or the 16-bit data value to be\n\nwritten to the PHY before a Management Write operation."]
    #[inline(always)]
    #[must_use]
    pub fn gd(&mut self) -> GdW<GmiiDataSpec> {
        GdW::new(self, 0)
    }
}
#[doc = "GMII Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmii_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmii_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmiiDataSpec;
impl crate::RegisterSpec for GmiiDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmii_data::R`](R) reader structure"]
impl crate::Readable for GmiiDataSpec {}
#[doc = "`write(|w| ..)` method takes [`gmii_data::W`](W) writer structure"]
impl crate::Writable for GmiiDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMII_DATA to value 0"]
impl crate::Resettable for GmiiDataSpec {
    const RESET_VALUE: u32 = 0;
}
