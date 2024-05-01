#[doc = "Register `BCSH_H` reader"]
pub type R = crate::R<BcshHSpec>;
#[doc = "Register `BCSH_H` writer"]
pub type W = crate::W<BcshHSpec>;
#[doc = "Field `SIN_HUE` reader - sin hue value"]
pub type SinHueR = crate::FieldReader<u16>;
#[doc = "Field `SIN_HUE` writer - sin hue value"]
pub type SinHueW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `COS_HUE` reader - cos hue value"]
pub type CosHueR = crate::FieldReader<u16>;
#[doc = "Field `COS_HUE` writer - cos hue value"]
pub type CosHueW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - sin hue value"]
    #[inline(always)]
    pub fn sin_hue(&self) -> SinHueR {
        SinHueR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - cos hue value"]
    #[inline(always)]
    pub fn cos_hue(&self) -> CosHueR {
        CosHueR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - sin hue value"]
    #[inline(always)]
    #[must_use]
    pub fn sin_hue(&mut self) -> SinHueW<BcshHSpec> {
        SinHueW::new(self, 0)
    }
    #[doc = "Bits 16:24 - cos hue value"]
    #[inline(always)]
    #[must_use]
    pub fn cos_hue(&mut self) -> CosHueW<BcshHSpec> {
        CosHueW::new(self, 16)
    }
}
#[doc = "Sin hue and cos hue config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcsh_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcsh_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcshHSpec;
impl crate::RegisterSpec for BcshHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcsh_h::R`](R) reader structure"]
impl crate::Readable for BcshHSpec {}
#[doc = "`write(|w| ..)` method takes [`bcsh_h::W`](W) writer structure"]
impl crate::Writable for BcshHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCSH_H to value 0x0100_0000"]
impl crate::Resettable for BcshHSpec {
    const RESET_VALUE: u32 = 0x0100_0000;
}
