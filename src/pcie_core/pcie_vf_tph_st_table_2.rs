#[doc = "Register `PCIE_VF_TPH_ST_TABLE_2` reader"]
pub type R = crate::R<PcieVfTphStTable2Spec>;
#[doc = "Register `PCIE_VF_TPH_ST_TABLE_2` writer"]
pub type W = crate::W<PcieVfTphStTable2Spec>;
#[doc = "Field `ST0L` reader - ST 0 Lower \\[ST0L\\]\n\nLower 8 bits of the first Steering Tag.\n\nThis is the 8- bit Steering Tag sent\n\nout in requests."]
pub type St0lR = crate::FieldReader;
#[doc = "Field `ST0L` writer - ST 0 Lower \\[ST0L\\]\n\nLower 8 bits of the first Steering Tag.\n\nThis is the 8- bit Steering Tag sent\n\nout in requests."]
pub type St0lW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ST0U` reader - ST 0 Upper \\[ST0U\\]\n\nThis field is used for the upper 8 bits\n\nof the first Steering Tag when\n\nExtended TPH Requester support is\n\nenabled."]
pub type St0uR = crate::FieldReader;
#[doc = "Field `ST1L` reader - ST 1 Lower \\[ST1L\\]\n\nLower 8 bits of the second Steering\n\nTag. This is the 8-bit Steering Tag\n\nsent out in requests."]
pub type St1lR = crate::FieldReader;
#[doc = "Field `ST1L` writer - ST 1 Lower \\[ST1L\\]\n\nLower 8 bits of the second Steering\n\nTag. This is the 8-bit Steering Tag\n\nsent out in requests."]
pub type St1lW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ST1U` reader - ST 1 Upper \\[ST1U\\]\n\nThis field is used for the upper 8 bits\n\nof the second Steering Tag when\n\nExtended TPH Requester support is\n\nenabled."]
pub type St1uR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - ST 0 Lower \\[ST0L\\]\n\nLower 8 bits of the first Steering Tag.\n\nThis is the 8- bit Steering Tag sent\n\nout in requests."]
    #[inline(always)]
    pub fn st0l(&self) -> St0lR {
        St0lR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ST 0 Upper \\[ST0U\\]\n\nThis field is used for the upper 8 bits\n\nof the first Steering Tag when\n\nExtended TPH Requester support is\n\nenabled."]
    #[inline(always)]
    pub fn st0u(&self) -> St0uR {
        St0uR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ST 1 Lower \\[ST1L\\]\n\nLower 8 bits of the second Steering\n\nTag. This is the 8-bit Steering Tag\n\nsent out in requests."]
    #[inline(always)]
    pub fn st1l(&self) -> St1lR {
        St1lR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ST 1 Upper \\[ST1U\\]\n\nThis field is used for the upper 8 bits\n\nof the second Steering Tag when\n\nExtended TPH Requester support is\n\nenabled."]
    #[inline(always)]
    pub fn st1u(&self) -> St1uR {
        St1uR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ST 0 Lower \\[ST0L\\]\n\nLower 8 bits of the first Steering Tag.\n\nThis is the 8- bit Steering Tag sent\n\nout in requests."]
    #[inline(always)]
    #[must_use]
    pub fn st0l(&mut self) -> St0lW<PcieVfTphStTable2Spec> {
        St0lW::new(self, 0)
    }
    #[doc = "Bits 16:23 - ST 1 Lower \\[ST1L\\]\n\nLower 8 bits of the second Steering\n\nTag. This is the 8-bit Steering Tag\n\nsent out in requests."]
    #[inline(always)]
    #[must_use]
    pub fn st1l(&mut self) -> St1lW<PcieVfTphStTable2Spec> {
        St1lW::new(self, 16)
    }
}
#[doc = "TPH ST Table 2\n\nThis field is used for the upper 8 bits\n\nof the second Steering Tag when\n\nExtended TPH Requester support is\n\nenabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_tph_st_table_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_tph_st_table_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfTphStTable2Spec;
impl crate::RegisterSpec for PcieVfTphStTable2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_tph_st_table_2::R`](R) reader structure"]
impl crate::Readable for PcieVfTphStTable2Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_vf_tph_st_table_2::W`](W) writer structure"]
impl crate::Writable for PcieVfTphStTable2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_VF_TPH_ST_TABLE_2 to value 0"]
impl crate::Resettable for PcieVfTphStTable2Spec {
    const RESET_VALUE: u32 = 0;
}
