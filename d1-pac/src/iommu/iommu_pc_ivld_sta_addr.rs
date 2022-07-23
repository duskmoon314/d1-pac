#[doc = "Register `iommu_pc_ivld_sta_addr` reader"]
pub struct R(crate::R<IOMMU_PC_IVLD_STA_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_PC_IVLD_STA_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_PC_IVLD_STA_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_PC_IVLD_STA_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_pc_ivld_sta_addr` writer"]
pub struct W(crate::W<IOMMU_PC_IVLD_STA_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_PC_IVLD_STA_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IOMMU_PC_IVLD_STA_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_PC_IVLD_STA_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pc_ivld_sa` reader - PTW Cache invalid start address, 1 MB aligned."]
pub type PC_IVLD_SA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pc_ivld_sa` writer - PTW Cache invalid start address, 1 MB aligned."]
pub type PC_IVLD_SA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IOMMU_PC_IVLD_STA_ADDR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 20:31 - PTW Cache invalid start address, 1 MB aligned."]
    #[inline(always)]
    pub fn pc_ivld_sa(&self) -> PC_IVLD_SA_R {
        PC_IVLD_SA_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:31 - PTW Cache invalid start address, 1 MB aligned."]
    #[inline(always)]
    pub fn pc_ivld_sa(&mut self) -> PC_IVLD_SA_W<20> {
        PC_IVLD_SA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU PC Invalidation Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_pc_ivld_sta_addr](index.html) module"]
pub struct IOMMU_PC_IVLD_STA_ADDR_SPEC;
impl crate::RegisterSpec for IOMMU_PC_IVLD_STA_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_pc_ivld_sta_addr::R](R) reader structure"]
impl crate::Readable for IOMMU_PC_IVLD_STA_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_pc_ivld_sta_addr::W](W) writer structure"]
impl crate::Writable for IOMMU_PC_IVLD_STA_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets iommu_pc_ivld_sta_addr to value 0"]
impl crate::Resettable for IOMMU_PC_IVLD_STA_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
