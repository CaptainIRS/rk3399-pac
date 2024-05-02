#[doc = "Register `MIPI_ADD_DATA_SEL_2` reader"]
pub type R = crate::R<MipiAddDataSel2Spec>;
#[doc = "Register `MIPI_ADD_DATA_SEL_2` writer"]
pub type W = crate::W<MipiAddDataSel2Spec>;
#[doc = "Field `ADD_DATA_TYPE_2` reader - data type selector for additional data output\n\n"]
pub type AddDataType2R = crate::FieldReader;
#[doc = "Field `ADD_DATA_TYPE_2` writer - data type selector for additional data output\n\n"]
pub type AddDataType2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ADD_DATA_VC_2` reader - virtual channel selector for additional data output"]
pub type AddDataVc2R = crate::FieldReader;
#[doc = "Field `ADD_DATA_VC_2` writer - virtual channel selector for additional data output"]
pub type AddDataVc2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - data type selector for additional data output\n\n"]
    #[inline(always)]
    pub fn add_data_type_2(&self) -> AddDataType2R {
        AddDataType2R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - virtual channel selector for additional data output"]
    #[inline(always)]
    pub fn add_data_vc_2(&self) -> AddDataVc2R {
        AddDataVc2R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - data type selector for additional data output\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn add_data_type_2(&mut self) -> AddDataType2W<MipiAddDataSel2Spec> {
        AddDataType2W::new(self, 0)
    }
    #[doc = "Bits 6:7 - virtual channel selector for additional data output"]
    #[inline(always)]
    #[must_use]
    pub fn add_data_vc_2(&mut self) -> AddDataVc2W<MipiAddDataSel2Spec> {
        AddDataVc2W::new(self, 6)
    }
}
#[doc = "Additional Data Selector 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_add_data_sel_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_add_data_sel_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiAddDataSel2Spec;
impl crate::RegisterSpec for MipiAddDataSel2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_add_data_sel_2::R`](R) reader structure"]
impl crate::Readable for MipiAddDataSel2Spec {}
#[doc = "`write(|w| ..)` method takes [`mipi_add_data_sel_2::W`](W) writer structure"]
impl crate::Writable for MipiAddDataSel2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIPI_ADD_DATA_SEL_2 to value 0xff"]
impl crate::Resettable for MipiAddDataSel2Spec {
    const RESET_VALUE: u32 = 0xff;
}
