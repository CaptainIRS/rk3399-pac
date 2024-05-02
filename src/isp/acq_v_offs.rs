#[doc = "Register `ACQ_V_OFFS` reader"]
pub type R = crate::R<AcqVOffsSpec>;
#[doc = "Register `ACQ_V_OFFS` writer"]
pub type W = crate::W<AcqVOffsSpec>;
#[doc = "Field `ACQ_V_OFFS` reader - vertical sample offset in lines\n\n"]
pub type AcqVOffsR = crate::FieldReader<u16>;
#[doc = "Field `ACQ_V_OFFS` writer - vertical sample offset in lines\n\n"]
pub type AcqVOffsW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - vertical sample offset in lines\n\n"]
    #[inline(always)]
    pub fn acq_v_offs(&self) -> AcqVOffsR {
        AcqVOffsR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - vertical sample offset in lines\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn acq_v_offs(&mut self) -> AcqVOffsW<AcqVOffsSpec> {
        AcqVOffsW::new(self, 0)
    }
}
#[doc = "vertical input offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acq_v_offs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acq_v_offs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcqVOffsSpec;
impl crate::RegisterSpec for AcqVOffsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acq_v_offs::R`](R) reader structure"]
impl crate::Readable for AcqVOffsSpec {}
#[doc = "`write(|w| ..)` method takes [`acq_v_offs::W`](W) writer structure"]
impl crate::Writable for AcqVOffsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACQ_V_OFFS to value 0"]
impl crate::Resettable for AcqVOffsSpec {
    const RESET_VALUE: u32 = 0;
}
