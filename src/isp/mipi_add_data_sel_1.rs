#[doc = "Register `MIPI_ADD_DATA_SEL_1` reader"]
pub type R = crate::R<MipiAddDataSel1Spec>;
#[doc = "Register `MIPI_ADD_DATA_SEL_1` writer"]
pub type W = crate::W<MipiAddDataSel1Spec>;
#[doc = "Field `ADD_DATA_TYPE_1` reader - data type selector for additional data output\n\n"]
pub type AddDataType1R = crate::FieldReader;
#[doc = "Field `ADD_DATA_TYPE_1` writer - data type selector for additional data output\n\n"]
pub type AddDataType1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ADD_DATA_VC_1` reader - virtual channel selector for additional data output"]
pub type AddDataVc1R = crate::FieldReader;
#[doc = "Field `ADD_DATA_VC_1` writer - virtual channel selector for additional data output"]
pub type AddDataVc1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - data type selector for additional data output\n\n"]
    #[inline(always)]
    pub fn add_data_type_1(&self) -> AddDataType1R {
        AddDataType1R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - virtual channel selector for additional data output"]
    #[inline(always)]
    pub fn add_data_vc_1(&self) -> AddDataVc1R {
        AddDataVc1R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - data type selector for additional data output\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn add_data_type_1(&mut self) -> AddDataType1W<MipiAddDataSel1Spec> {
        AddDataType1W::new(self, 0)
    }
    #[doc = "Bits 6:7 - virtual channel selector for additional data output"]
    #[inline(always)]
    #[must_use]
    pub fn add_data_vc_1(&mut self) -> AddDataVc1W<MipiAddDataSel1Spec> {
        AddDataVc1W::new(self, 6)
    }
}
#[doc = "Additional Data Selector 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_add_data_sel_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_add_data_sel_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiAddDataSel1Spec;
impl crate::RegisterSpec for MipiAddDataSel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_add_data_sel_1::R`](R) reader structure"]
impl crate::Readable for MipiAddDataSel1Spec {}
#[doc = "`write(|w| ..)` method takes [`mipi_add_data_sel_1::W`](W) writer structure"]
impl crate::Writable for MipiAddDataSel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIPI_ADD_DATA_SEL_1 to value 0xff"]
impl crate::Resettable for MipiAddDataSel1Spec {
    const RESET_VALUE: u32 = 0xff;
}
