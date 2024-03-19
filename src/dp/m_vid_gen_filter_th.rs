#[doc = "Register `M_VID_GEN_FILTER_TH` reader"]
pub type R = crate::R<MVidGenFilterThSpec>;
#[doc = "Register `M_VID_GEN_FILTER_TH` writer"]
pub type W = crate::W<MVidGenFilterThSpec>;
#[doc = "Field `M_VID_GEN_FILTER_TH` reader - The threshold of M_VID generation filter \n\nIt only takes effect when \n\nM_VID_GEN_FILTER_EN is set to 1"]
pub type MVidGenFilterThR = crate::FieldReader;
#[doc = "Field `M_VID_GEN_FILTER_TH` writer - The threshold of M_VID generation filter \n\nIt only takes effect when \n\nM_VID_GEN_FILTER_EN is set to 1"]
pub type MVidGenFilterThW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The threshold of M_VID generation filter \n\nIt only takes effect when \n\nM_VID_GEN_FILTER_EN is set to 1"]
    #[inline(always)]
    pub fn m_vid_gen_filter_th(&self) -> MVidGenFilterThR {
        MVidGenFilterThR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The threshold of M_VID generation filter \n\nIt only takes effect when \n\nM_VID_GEN_FILTER_EN is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn m_vid_gen_filter_th(&mut self) -> MVidGenFilterThW<MVidGenFilterThSpec> {
        MVidGenFilterThW::new(self, 0)
    }
}
#[doc = "DP M_VID Value Calculation Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m_vid_gen_filter_th::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_vid_gen_filter_th::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MVidGenFilterThSpec;
impl crate::RegisterSpec for MVidGenFilterThSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m_vid_gen_filter_th::R`](R) reader structure"]
impl crate::Readable for MVidGenFilterThSpec {}
#[doc = "`write(|w| ..)` method takes [`m_vid_gen_filter_th::W`](W) writer structure"]
impl crate::Writable for MVidGenFilterThSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M_VID_GEN_FILTER_TH to value 0x04"]
impl crate::Resettable for MVidGenFilterThSpec {
    const RESET_VALUE: u32 = 0x04;
}
