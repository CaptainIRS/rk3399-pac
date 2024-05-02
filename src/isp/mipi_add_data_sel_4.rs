#[doc = "Register `MIPI_ADD_DATA_SEL_4` reader"]
pub type R = crate::R<MipiAddDataSel4Spec>;
#[doc = "Register `MIPI_ADD_DATA_SEL_4` writer"]
pub type W = crate::W<MipiAddDataSel4Spec>;
#[doc = "Field `ADD_DATA_TYPE_4` reader - data type selector for additional data output\n\n"]
pub type AddDataType4R = crate::FieldReader;
#[doc = "Field `ADD_DATA_TYPE_4` writer - data type selector for additional data output\n\n"]
pub type AddDataType4W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ADD_DATA_VC_4` reader - virtual channel selector for additional data output\n\n\n\n"]
pub type AddDataVc4R = crate::FieldReader;
#[doc = "Field `ADD_DATA_VC_4` writer - virtual channel selector for additional data output\n\n\n\n"]
pub type AddDataVc4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - data type selector for additional data output\n\n"]
    #[inline(always)]
    pub fn add_data_type_4(&self) -> AddDataType4R {
        AddDataType4R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - virtual channel selector for additional data output\n\n\n\n"]
    #[inline(always)]
    pub fn add_data_vc_4(&self) -> AddDataVc4R {
        AddDataVc4R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - data type selector for additional data output\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn add_data_type_4(&mut self) -> AddDataType4W<MipiAddDataSel4Spec> {
        AddDataType4W::new(self, 0)
    }
    #[doc = "Bits 6:7 - virtual channel selector for additional data output\n\n\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn add_data_vc_4(&mut self) -> AddDataVc4W<MipiAddDataSel4Spec> {
        AddDataVc4W::new(self, 6)
    }
}
#[doc = "Additional Data Selector 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_add_data_sel_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_add_data_sel_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiAddDataSel4Spec;
impl crate::RegisterSpec for MipiAddDataSel4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_add_data_sel_4::R`](R) reader structure"]
impl crate::Readable for MipiAddDataSel4Spec {}
#[doc = "`write(|w| ..)` method takes [`mipi_add_data_sel_4::W`](W) writer structure"]
impl crate::Writable for MipiAddDataSel4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIPI_ADD_DATA_SEL_4 to value 0xff"]
impl crate::Resettable for MipiAddDataSel4Spec {
    const RESET_VALUE: u32 = 0xff;
}
