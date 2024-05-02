#[doc = "Register `MIPI_ADD_DATA_SEL_3` reader"]
pub type R = crate::R<MipiAddDataSel3Spec>;
#[doc = "Register `MIPI_ADD_DATA_SEL_3` writer"]
pub type W = crate::W<MipiAddDataSel3Spec>;
#[doc = "Field `ADD_DATA_TYPE_3` reader - data type selector for additional data output\n\n"]
pub type AddDataType3R = crate::FieldReader;
#[doc = "Field `ADD_DATA_TYPE_3` writer - data type selector for additional data output\n\n"]
pub type AddDataType3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ADD_DATA_VC_3` reader - virtual channel selector for additional data output"]
pub type AddDataVc3R = crate::FieldReader;
#[doc = "Field `ADD_DATA_VC_3` writer - virtual channel selector for additional data output"]
pub type AddDataVc3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - data type selector for additional data output\n\n"]
    #[inline(always)]
    pub fn add_data_type_3(&self) -> AddDataType3R {
        AddDataType3R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - virtual channel selector for additional data output"]
    #[inline(always)]
    pub fn add_data_vc_3(&self) -> AddDataVc3R {
        AddDataVc3R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - data type selector for additional data output\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn add_data_type_3(&mut self) -> AddDataType3W<MipiAddDataSel3Spec> {
        AddDataType3W::new(self, 0)
    }
    #[doc = "Bits 6:7 - virtual channel selector for additional data output"]
    #[inline(always)]
    #[must_use]
    pub fn add_data_vc_3(&mut self) -> AddDataVc3W<MipiAddDataSel3Spec> {
        AddDataVc3W::new(self, 6)
    }
}
#[doc = "Additional Data Selector 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_add_data_sel_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_add_data_sel_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiAddDataSel3Spec;
impl crate::RegisterSpec for MipiAddDataSel3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_add_data_sel_3::R`](R) reader structure"]
impl crate::Readable for MipiAddDataSel3Spec {}
#[doc = "`write(|w| ..)` method takes [`mipi_add_data_sel_3::W`](W) writer structure"]
impl crate::Writable for MipiAddDataSel3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIPI_ADD_DATA_SEL_3 to value 0xff"]
impl crate::Resettable for MipiAddDataSel3Spec {
    const RESET_VALUE: u32 = 0xff;
}
