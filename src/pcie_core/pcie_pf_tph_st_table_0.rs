#[doc = "Register `PCIE_PF_TPH_ST_TABLE_0` reader"]
pub type R = crate::R<PciePfTphStTable0Spec>;
#[doc = "Register `PCIE_PF_TPH_ST_TABLE_0` writer"]
pub type W = crate::W<PciePfTphStTable0Spec>;
#[doc = "Field `STL0` reader - ST Lower 0 \\[STL0\\]\n\nLower 8 bits of the first Steering Tag.\n\nThis is the 8- bit Steering Tag sent\n\nout in requests."]
pub type Stl0R = crate::FieldReader;
#[doc = "Field `STL0` writer - ST Lower 0 \\[STL0\\]\n\nLower 8 bits of the first Steering Tag.\n\nThis is the 8- bit Steering Tag sent\n\nout in requests."]
pub type Stl0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `STU0` reader - ST Upper 0 \\[STU0\\]\n\nThis field is used for the upper 8 bits\n\nof the first Steering Tag when\n\nExtended TPH Requester support is\n\nenabled."]
pub type Stu0R = crate::FieldReader;
#[doc = "Field `STL1` reader - ST Lower 1 \\[STL1\\]\n\nLower 8 bits of the second Steering\n\nTag. This is the 8-bit Steering Tag\n\nsent out in requests."]
pub type Stl1R = crate::FieldReader;
#[doc = "Field `STL1` writer - ST Lower 1 \\[STL1\\]\n\nLower 8 bits of the second Steering\n\nTag. This is the 8-bit Steering Tag\n\nsent out in requests."]
pub type Stl1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `STU1` reader - ST Upper 1 \\[STU1\\]\n\nThis field is used for the upper 8 bits\n\nof the second Steering Tag when\n\nExtended TPH Requester support is\n\nenabled."]
pub type Stu1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - ST Lower 0 \\[STL0\\]\n\nLower 8 bits of the first Steering Tag.\n\nThis is the 8- bit Steering Tag sent\n\nout in requests."]
    #[inline(always)]
    pub fn stl0(&self) -> Stl0R {
        Stl0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ST Upper 0 \\[STU0\\]\n\nThis field is used for the upper 8 bits\n\nof the first Steering Tag when\n\nExtended TPH Requester support is\n\nenabled."]
    #[inline(always)]
    pub fn stu0(&self) -> Stu0R {
        Stu0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ST Lower 1 \\[STL1\\]\n\nLower 8 bits of the second Steering\n\nTag. This is the 8-bit Steering Tag\n\nsent out in requests."]
    #[inline(always)]
    pub fn stl1(&self) -> Stl1R {
        Stl1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ST Upper 1 \\[STU1\\]\n\nThis field is used for the upper 8 bits\n\nof the second Steering Tag when\n\nExtended TPH Requester support is\n\nenabled."]
    #[inline(always)]
    pub fn stu1(&self) -> Stu1R {
        Stu1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ST Lower 0 \\[STL0\\]\n\nLower 8 bits of the first Steering Tag.\n\nThis is the 8- bit Steering Tag sent\n\nout in requests."]
    #[inline(always)]
    #[must_use]
    pub fn stl0(&mut self) -> Stl0W<PciePfTphStTable0Spec> {
        Stl0W::new(self, 0)
    }
    #[doc = "Bits 16:23 - ST Lower 1 \\[STL1\\]\n\nLower 8 bits of the second Steering\n\nTag. This is the 8-bit Steering Tag\n\nsent out in requests."]
    #[inline(always)]
    #[must_use]
    pub fn stl1(&mut self) -> Stl1W<PciePfTphStTable0Spec> {
        Stl1W::new(self, 16)
    }
}
#[doc = "TPH ST Table 0\n\nThis field is used for the upper 8 bits\n\nof the second Steering Tag when\n\nExtended TPH Requester support is\n\nenabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_tph_st_table_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_tph_st_table_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfTphStTable0Spec;
impl crate::RegisterSpec for PciePfTphStTable0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_tph_st_table_0::R`](R) reader structure"]
impl crate::Readable for PciePfTphStTable0Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_tph_st_table_0::W`](W) writer structure"]
impl crate::Writable for PciePfTphStTable0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_TPH_ST_TABLE_0 to value 0"]
impl crate::Resettable for PciePfTphStTable0Spec {
    const RESET_VALUE: u32 = 0;
}
