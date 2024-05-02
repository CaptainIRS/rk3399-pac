#[doc = "Register `IS_H_OFFS` reader"]
pub type R = crate::R<IsHOffsSpec>;
#[doc = "Register `IS_H_OFFS` writer"]
pub type W = crate::W<IsHOffsSpec>;
#[doc = "Field `is_h_offs` reader - horizontal picture offset in pixel\n\n"]
pub type IsHOffsR = crate::FieldReader<u16>;
#[doc = "Field `is_h_offs` writer - horizontal picture offset in pixel\n\n"]
pub type IsHOffsW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - horizontal picture offset in pixel\n\n"]
    #[inline(always)]
    pub fn is_h_offs(&self) -> IsHOffsR {
        IsHOffsR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - horizontal picture offset in pixel\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn is_h_offs(&mut self) -> IsHOffsW<IsHOffsSpec> {
        IsHOffsW::new(self, 0)
    }
}
#[doc = "Horizontal offset of output window\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_h_offs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is_h_offs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsHOffsSpec;
impl crate::RegisterSpec for IsHOffsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`is_h_offs::R`](R) reader structure"]
impl crate::Readable for IsHOffsSpec {}
#[doc = "`write(|w| ..)` method takes [`is_h_offs::W`](W) writer structure"]
impl crate::Writable for IsHOffsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IS_H_OFFS to value 0"]
impl crate::Resettable for IsHOffsSpec {
    const RESET_VALUE: u32 = 0;
}
