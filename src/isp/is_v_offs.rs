#[doc = "Register `IS_V_OFFS` reader"]
pub type R = crate::R<IsVOffsSpec>;
#[doc = "Register `IS_V_OFFS` writer"]
pub type W = crate::W<IsVOffsSpec>;
#[doc = "Field `is_v_offs` reader - vertical picture offset in lines"]
pub type IsVOffsR = crate::FieldReader<u16>;
#[doc = "Field `is_v_offs` writer - vertical picture offset in lines"]
pub type IsVOffsW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - vertical picture offset in lines"]
    #[inline(always)]
    pub fn is_v_offs(&self) -> IsVOffsR {
        IsVOffsR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - vertical picture offset in lines"]
    #[inline(always)]
    #[must_use]
    pub fn is_v_offs(&mut self) -> IsVOffsW<IsVOffsSpec> {
        IsVOffsW::new(self, 0)
    }
}
#[doc = "Vertical offset of output window\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_v_offs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is_v_offs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsVOffsSpec;
impl crate::RegisterSpec for IsVOffsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`is_v_offs::R`](R) reader structure"]
impl crate::Readable for IsVOffsSpec {}
#[doc = "`write(|w| ..)` method takes [`is_v_offs::W`](W) writer structure"]
impl crate::Writable for IsVOffsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IS_V_OFFS to value 0"]
impl crate::Resettable for IsVOffsSpec {
    const RESET_VALUE: u32 = 0;
}
