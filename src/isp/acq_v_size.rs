#[doc = "Register `ACQ_V_SIZE` reader"]
pub type R = crate::R<AcqVSizeSpec>;
#[doc = "Register `ACQ_V_SIZE` writer"]
pub type W = crate::W<AcqVSizeSpec>;
#[doc = "Field `ACQ_V_SIZE` reader - vertical sample size in lines\n\n"]
pub type AcqVSizeR = crate::FieldReader<u16>;
#[doc = "Field `ACQ_V_SIZE` writer - vertical sample size in lines\n\n"]
pub type AcqVSizeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - vertical sample size in lines\n\n"]
    #[inline(always)]
    pub fn acq_v_size(&self) -> AcqVSizeR {
        AcqVSizeR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - vertical sample size in lines\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn acq_v_size(&mut self) -> AcqVSizeW<AcqVSizeSpec> {
        AcqVSizeW::new(self, 0)
    }
}
#[doc = "vertical input size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acq_v_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acq_v_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcqVSizeSpec;
impl crate::RegisterSpec for AcqVSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acq_v_size::R`](R) reader structure"]
impl crate::Readable for AcqVSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`acq_v_size::W`](W) writer structure"]
impl crate::Writable for AcqVSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACQ_V_SIZE to value 0x0c00"]
impl crate::Resettable for AcqVSizeSpec {
    const RESET_VALUE: u32 = 0x0c00;
}
